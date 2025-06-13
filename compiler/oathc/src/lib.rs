use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    time::SystemTime,
};

use dashmap::*;
use derive_more::Display;
use oathc_ast::*;
use oathc_diagnostics::*;
use oathc_file::*;
use oathc_interner::*;
use oathc_tokenizer::*;

pub use oathc_diagnostics::{Diagnostic, Error, IdentCase, NameError, SyntaxError, SyntaxWarning, TokenError, Warning};
pub use oathc_highlighting::{Highlight, HighlightColor};
pub use oathc_span::{ConnectSpan, OptionSpanned, Position, Span, Spanned};
pub use oathc_tokens::KEYWORDS;

#[derive(Debug)]
pub struct OathCompiler {
    interner: Arc<Interner>,
    file_interner: Arc<FileInterner>,
    diagnostics: Diagnostics,
    libs: DashMap<LibId, Lib>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibId(usize);

impl OathCompiler {
    pub fn new() -> Self {
        Self {
            interner: Arc::new(Interner::new()),
            file_interner: Arc::new(FileInterner::new()),
            diagnostics: Diagnostics::new(),
            libs: DashMap::new(),
        }
    }

    pub fn create_lib(&self, dir_path: PathBuf) -> LibId {
        let id = (0..)
            .map(|i| LibId(i))
            .filter(|id| !self.libs.contains_key(id))
            .next()
            .unwrap();

        let root_mod_path = dir_path.join("mod.oh");

        let lib = Lib {
            root_mod: RwLock::new(if root_mod_path.is_file() {
                let mut file = File::open(&root_mod_path).unwrap();

                let mut mod_ = Mod {
                    path: root_mod_path,
                    time: file.metadata().unwrap().modified().unwrap(),
                    can_have_children: true,
                    ast: SyntaxTree::default(),
                    highlights: Vec::new(),
                };

                let mut text = String::new();
                file.read_to_string(&mut text).unwrap();
                self.update_mod(&mut mod_, &text);

                Ok(mod_)
            } else {
                Err(ModError::CantFind("mod.oh".to_string()))
            }),
            dir_path,
        };

        self.libs.insert(id, lib);

        id
    }

    pub fn check_libs(&self) {
        for lib in &self.libs {
            let root_mod_path = lib.dir_path.join("mod.oh");

            let changed = if root_mod_path.is_file() {
                let root_mod = lib.root_mod.read().unwrap();
                if let Ok(root_mod) = &*root_mod {
                    let file = File::open(&root_mod_path).unwrap();
                    let file_time = file.metadata().unwrap().modified().unwrap();

                    file_time > root_mod.time
                } else {
                    true
                }
            } else {
                lib.root_mod.read().unwrap().is_ok()
            };

            if changed {
                let mut file = File::open(&root_mod_path).unwrap();

                let mut mod_ = Mod {
                    path: root_mod_path.clone(),
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
        self.diagnostics
            .diagnostics()
            .map(|(file, diagnostics)| (PathBuf::from(self.file_interner.unintern(file)), diagnostics))
    }
    pub fn file_diagnostics(&self, file: impl AsRef<Path>) -> impl Iterator<Item = Diagnostic> {
        self.diagnostics.file_diagnostics(self.file_interner.intern(file))
    }
    pub fn dirty_diagnostics(&self) -> impl Iterator<Item = (PathBuf, impl Iterator<Item = Diagnostic>)> {
        self.diagnostics
            .dirty_files()
            .map(|(file_id, diagnostics)| (self.file_interner.unintern(file_id), diagnostics))
    }

    pub fn file_highligts(&self, file: impl AsRef<Path>) -> impl Iterator<Item = Highlight> {
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
        let fake_path = self.file_interner.intern("");
        let fake_diagnostics = Diagnostics::new();

        let ast = text
            .tokenize(fake_path, &self.interner, &fake_diagnostics, &mut Vec::new())
            .parse_ast();

        let output = fake_diagnostics.file_diagnostics(fake_path).collect();

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
    pub highlights: Vec<Highlight>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
enum ModError {
    CantFind(String),
}

impl OathCompiler {
    fn find_mod_file(&self, dir_path: &Path) -> Option<PathBuf> {
        let path = dir_path.join("mod.oh");

        path.is_file().then_some(path)
    }

    fn update_mod(&self, mod_: &mut Mod, text: &str) {
        mod_.highlights.clear();
        mod_.ast = text
            .tokenize(
                self.file_interner.intern(&mod_.path),
                &self.interner,
                &self.diagnostics,
                &mut mod_.highlights,
            )
            .parse_ast();
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
