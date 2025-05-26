use std::{
    fs::File,
    io::Read,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
};

use dashmap::*;
use derive_more::Display;
use oath_ast::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_nameres::*;
use oath_parse_context::*;
use oath_tokenizer::*;

pub use oath_diagnostics::*;
pub use oath_highlighting::{Highlight, HighlightColor, HighlightInfo};
pub use oath_src::*;
pub use oath_tokens::KEYWORDS;

#[derive(Debug)]
pub struct OathCompiler {
    interner: Arc<Interner>,
    libs: DashMap<LibId, Lib>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibId(usize);

impl OathCompiler {
    pub fn new() -> Self {
        Self {
            interner: Arc::new(Interner::new()),
            libs: DashMap::new(),
        }
    }

    pub fn create_lib(&self, dir_path: PathBuf, dependencies: impl IntoIterator<Item = LibId>) -> LibId {
        let id = (0..)
            .map(|i| LibId(i))
            .filter(|id| !self.libs.contains_key(id))
            .next()
            .unwrap();

        let lib = Lib {
            root_mod: self.find_root_mod(&dir_path),
            dir_path,
        };

        self.libs.insert(id, lib);

        id
    }

    pub fn get_lib_diagnostics(&self, lib: LibId) -> impl IntoIterator<Item = Diagnostic> {
        let lib = self.libs.get(&lib).unwrap();

        match &lib.root_mod {
            Ok(root_mod) => root_mod
                .parser_diagnostics
                .iter()
                .chain(root_mod.semantic_diagnostics.iter())
                .cloned()
                .collect(),
            Err(_) => Vec::new(),
        }
    }
    pub fn get_mod_semantic_diagnostics(&self, path: impl AsRef<Path>) -> impl Iterator<Item = Diagnostic> {
        let path = path.as_ref();

        for lib in self.libs.iter() {
            if path.parent().unwrap() == lib.dir_path
                && (path.file_name().unwrap() == "main" || path.file_name().unwrap() == "lib")
            {
                return match &lib.root_mod {
                    Ok(root_mod) => root_mod.semantic_diagnostics.iter().cloned().collect(),
                    Err(_) => Vec::new(),
                }
                .into_iter();
            }
        }

        Vec::new().into_iter()
    }

    pub fn get_mod_highligts(&self, path: impl AsRef<Path>) -> impl Iterator<Item = HighlightInfo> {
        let path = path.as_ref();

        for lib in self.libs.iter() {
            if path.parent().unwrap() == lib.dir_path
                && (path.file_name().unwrap() == "main" || path.file_name().unwrap() == "lib")
            {
                return match &lib.root_mod {
                    Ok(root_mod) => root_mod.highlights.iter().cloned().collect(),
                    Err(_) => Vec::new(),
                }
                .into_iter();
            }
        }

        Vec::new().into_iter()
    }

    pub fn parse_text(&self, text: &str) -> Vec<Diagnostic> {
        let mut context = ParseContext {
            diagnostics: Vec::new(),
            highlighter: Highlighter::new(),
            interner: self.interner.clone(),
        };

        let _ = text.tokenize(&mut context).parse_ast();

        context.diagnostics
    }

    pub fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        diagnostic.to_string_interned(&self.interner)
    }
}

#[derive(Debug)]
struct Lib {
    pub dir_path: PathBuf,
    pub root_mod: Result<Mod, ModError>,
}

#[derive(Debug)]
struct Mod {
    pub dir_path: PathBuf,
    pub can_have_children: bool,
    pub parser_diagnostics: Vec<Diagnostic>,
    pub semantic_diagnostics: Vec<Diagnostic>,
    pub highlights: Vec<HighlightInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
enum ModError {
    CantFind(String),
    FoundBoth(String),
}

impl OathCompiler {
    fn find_root_mod(&self, dir_path: &Path) -> Result<Mod, ModError> {
        let main_file = File::open(dir_path.join("main.oh")).ok();
        let lib_file = File::open(dir_path.join("lib.oh")).ok();

        let mut file = match (main_file, lib_file) {
            (Some(main_file), None) => main_file,
            (None, Some(lib_file)) => lib_file,
            (Some(_), Some(_)) => {
                return Err(ModError::FoundBoth(format!(
                    "found both `main.oh` and `lib.oh` in {}",
                    dir_path.display()
                )));
            }
            (None, None) => {
                return Err(ModError::CantFind(format!(
                    "can't find `main.oh` or `lib.oh` in {}",
                    dir_path.display()
                )));
            }
        };

        let mut output = Mod {
            dir_path: dir_path.to_path_buf(),
            can_have_children: true,
            parser_diagnostics: Vec::new(),
            semantic_diagnostics: Vec::new(),
            highlights: Vec::new(),
        };

        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        self.update_mod(&mut output, &text);

        Ok(output)
    }

    fn find_child_mod(&self, parent: &Mod, name: StrId) -> Result<Mod, ModError> {
        let name = self.interner.unintern(name);

        let file_file = File::open(parent.dir_path.join(format!("{name}.oh"))).ok();
        let dir_file = File::open(parent.dir_path.join(&name).join("mod.oh")).ok();

        let (mut file, dir_path) = match (file_file, dir_file) {
            (Some(file_file), None) => (file_file, parent.dir_path.clone()),
            (None, Some(dir_file)) => (dir_file, parent.dir_path.join(&name)),
            (Some(_), Some(_)) => {
                return Err(ModError::FoundBoth(format!(
                    "found both `{name}.oh` and `{name}/mod.oh` in {}",
                    parent.dir_path.display()
                )));
            }
            (None, None) => {
                return Err(ModError::CantFind(format!(
                    "can't find `{name}.oh` or `{name}/mod.oh` in {}",
                    parent.dir_path.display()
                )));
            }
        };

        let mut output = Mod {
            dir_path,
            can_have_children: true,
            parser_diagnostics: Vec::new(),
            semantic_diagnostics: Vec::new(),
            highlights: Vec::new(),
        };

        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        self.update_mod(&mut output, &text);

        Ok(output)
    }

    fn update_mod(&self, mod_: &mut Mod, text: &str) {
        let mut context = ParseContext {
            diagnostics: Vec::new(),
            highlighter: Highlighter::new(),
            interner: self.interner.clone(),
        };

        let ast = text.tokenize(&mut context).parse_ast();
        mod_.parser_diagnostics = take(&mut context.diagnostics);

        ast.nameres(&mut context);
        mod_.semantic_diagnostics = context.diagnostics;
    }
}
