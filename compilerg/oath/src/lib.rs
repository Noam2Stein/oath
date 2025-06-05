use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    time::SystemTime,
};

use dashmap::*;
use derive_more::Display;
use oath_ast::*;
use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_parse_context::*;
use oath_tokenizer::*;

pub use oath_diagnostics::{Diagnostic, Error, IdentCase, NameError, SyntaxError, SyntaxWarning, TokenError, Warning};
pub use oath_highlighting::{Highlight, HighlightColor, HighlightInfo};
pub use oath_src::*;
pub use oath_tokens::KEYWORDS;

#[derive(Debug)]
pub struct OathCompiler {
    interner: Arc<Interner>,
    diagnostics: Diagnostics,
    libs: DashMap<LibId, Lib>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibId(usize);

impl OathCompiler {
    pub fn new() -> Self {
        Self {
            interner: Arc::new(Interner::new()),
            diagnostics: Diagnostics::new(),
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
            root_mod: RwLock::new(self.find_root_mod_path(&dir_path).map(|path| {
                let mut file = File::open(&path).unwrap();

                let mut mod_ = Mod {
                    path,
                    time: file.metadata().unwrap().modified().unwrap(),
                    can_have_children: true,
                    ast: SyntaxTree::default(),
                    highlights: Vec::new(),
                };

                let mut text = String::new();
                file.read_to_string(&mut text).unwrap();
                self.update_mod(&mut mod_, &text);

                mod_
            })),
            dir_path,
        };

        self.libs.insert(id, lib);

        id
    }

    pub fn check_libs(&self) {
        for lib in &self.libs {
            let root_path = self.find_root_mod_path(&lib.dir_path);

            let changed_path = match (root_path, lib.root_mod.read().unwrap().as_ref()) {
                (Ok(path), Ok(mod_)) => {
                    if path != mod_.path {
                        Some(path)
                    } else {
                        let file = File::open(&path).unwrap();
                        let file_time = file.metadata().unwrap().modified().unwrap();

                        if file_time > mod_.time { Some(path.clone()) } else { None }
                    }
                }
                (Ok(path), Err(_)) => Some(path),
                (Err(_), Ok(_)) => None,
                (Err(_), Err(_)) => None,
            };

            if let Some(changed_path) = changed_path {
                let mut file = File::open(&changed_path).unwrap();

                let mut mod_ = Mod {
                    path: changed_path.clone(),
                    time: file.metadata().unwrap().modified().unwrap(),
                    can_have_children: true,
                    ast: SyntaxTree::default(),
                    highlights: Vec::new(),
                };

                let mut text = String::new();
                file.read_to_string(&mut text).unwrap();
                self.update_mod(&mut mod_, &text);

                *lib.root_mod.write().unwrap() = Ok(mod_);
            }
        }
    }

    pub fn diagnostics(&self) -> impl IntoIterator<Item = (PathBuf, impl Iterator<Item = Diagnostic>)> {
        self.diagnostics.diagnostics()
    }
    pub fn file_diagnostics(&self, file: impl AsRef<Path>) -> impl Iterator<Item = Diagnostic> {
        self.diagnostics.file_diagnostics(file)
    }
    pub fn dirty_diagnostics(&self) -> impl IntoIterator<Item = (PathBuf, impl Iterator<Item = Diagnostic>)> {
        self.diagnostics.dirty_files()
    }

    pub fn file_highligts(&self, file: impl AsRef<Path>) -> impl Iterator<Item = HighlightInfo> {
        let path = file.as_ref();

        for lib in self.libs.iter() {
            if path.parent().unwrap() == lib.dir_path
                && (path.file_name().unwrap() == "main.oh" || path.file_name().unwrap() == "lib.oh")
            {
                return match &lib.root_mod.read().unwrap().as_ref() {
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
            path: PathBuf::new(),
            interner: self.interner.clone(),
            diagnostics: Diagnostics::new(),
            highlighter: Highlighter::new(),
        };

        let ast = text.tokenize(&mut context).parse_ast();
        let output = context.diagnostics.file_diagnostics(&PathBuf::new()).collect();

        drop(ast);

        output
    }

    pub fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        diagnostic.to_string_interned(&self.interner)
    }
}

#[derive(Debug)]
struct Lib {
    pub dir_path: PathBuf,
    pub root_mod: RwLock<Result<Mod, ModError>>,
}

#[derive(Debug)]
struct Mod {
    pub time: SystemTime,
    pub path: PathBuf,
    pub can_have_children: bool,
    pub ast: SyntaxTree,
    pub highlights: Vec<HighlightInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
enum ModError {
    CantFind(String),
    FoundBoth(String),
}

impl OathCompiler {
    fn find_root_mod_path(&self, dir_path: &Path) -> Result<PathBuf, ModError> {
        let main_path = dir_path.join("main.oh");
        let lib_path = dir_path.join("lib.oh");

        let main_path = if main_path.exists() { Some(main_path) } else { None };
        let lib_path = if lib_path.exists() { Some(lib_path) } else { None };

        let path = match (main_path, lib_path) {
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

        Ok(path)
    }

    fn update_mod(&self, mod_: &mut Mod, text: &str) {
        let mut context = ParseContext {
            path: mod_.path.clone(),
            highlighter: Highlighter::new(),
            diagnostics: self.diagnostics.arc_clone(),
            interner: self.interner.clone(),
        };

        mod_.ast = text.tokenize(&mut context).parse_ast();
        mod_.highlights = context.highlighter.highlights;
    }
}

/*
fn find_child_mod(&self, parent: &Mod, name: StrId) -> Result<Mod, ModError> {
        let name = self.interner.unintern(name);

        let file_file = File::open(parent.path.parent().unwrap().join(format!("{name}.oh"))).ok();
        let dir_file = File::open(parent.path.parent().unwrap().join(&name).join("mod.oh")).ok();

        let path = match (file_file, dir_file) {
            (Some(file_file), None) => (file_file, parent.path.parent().unwrap().to_path_buf()),
            (None, Some(dir_file)) => (dir_file, parent.path.parent().unwrap().join(&name)),
            (Some(_), Some(_)) => {
                return Err(ModError::FoundBoth(format!(
                    "found both `{name}.oh` and `{name}/mod.oh` in {}",
                    parent.path.parent().unwrap().display()
                )));
            }
            (None, None) => {
                return Err(ModError::CantFind(format!(
                    "can't find `{name}.oh` or `{name}/mod.oh` in {}",
                    parent.path.parent().unwrap().display()
                )));
            }
        };

        let mut output = Mod {
            path,
            can_have_children: true,
            parser_diagnostics: Vec::new(),
            semantic_diagnostics: Vec::new(),
            highlights: Vec::new(),
        };

        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        self.update_mod(&mut output, &text);

        Ok(output)
    } */
