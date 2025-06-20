use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use dashmap::*;
use oathc_ast::ParseAstExt;
use oathc_diagnostics::*;
use oathc_file::*;
use oathc_interner::*;
use oathc_res::*;
use oathc_tokenizer::*;
use oathc_tokens::*;

pub use oathc_diagnostics::{Diagnostic, Error, IdentCase, Warning};
pub use oathc_highlighting::{Highlight, HighlightColor};
pub use oathc_span::{ConnectSpan, OptionSpanned, Position, Span, Spanned};
pub use oathc_tokens::KEYWORDS;

#[derive(Debug)]
pub struct OathCompiler {
    interner: Arc<Interner>,
    file_interner: Arc<FileInterner>,
    diagnostics: Diagnostics,
    libs: DashMap<LibId, ModFile>,
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

    pub fn create_lib(&self, dir_path: PathBuf, responsible_file: impl AsRef<Path>) -> LibId {
        let id = (0..)
            .map(|i| LibId(i))
            .filter(|id| !self.libs.contains_key(id))
            .next()
            .unwrap();

        let name = Ident::new(
            "src",
            Span::from_range(self.file_interner.intern(responsible_file), 0, 0, 0, 0),
            &self.interner,
        )
        .unwrap();

        let mod_ = ModFile::new(dir_path, name, &self.interner, &self.file_interner, &self.diagnostics);

        self.libs.insert(id, mod_);

        id
    }

    pub fn check_lib_changes(&self) {
        for mut lib in self.libs.iter_mut() {
            lib.check(&self.interner, &self.file_interner, &self.diagnostics);
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
            if let Some(mod_) = lib.find(path) {
                return mod_.get_highlights().iter().copied().collect::<Vec<_>>().into_iter();
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
