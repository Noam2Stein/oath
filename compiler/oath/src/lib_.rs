use std::collections::HashMap;

use dashmap::DashMap;
use oath_ast::ParseAstExt;
use oath_nameres::NameresExt;
use oath_parse_context::ParseContext;

use super::*;

pub struct Lib {
    interner: Arc<Interner>,
    mods: DashMap<ModPath, (Vec<Diagnostic>, Vec<(Span, HighlightColor)>)>,
}

impl Lib {
    pub(super) fn new(oath: &Oath, mods: HashMap<ModPath, impl AsRef<str>>) -> Self {
        Self {
            interner: oath.interner.clone(),
            mods: mods
                .into_iter()
                .map(|(key, content)| (key, parse(content.as_ref(), oath.interner.clone())))
                .collect(),
        }
    }

    pub fn write_mod(&self, path: &ModPath, content: &str) {
        self.mods.insert(path.clone(), parse(content, self.interner.clone()));
    }
    pub fn delete_mod(&self, path: &ModPath) {
        self.mods.remove(path);
    }

    pub fn get_diagnostics(&self) -> impl Iterator<Item = Diagnostic> {
        let mut output = Vec::new();

        for mod_ in &self.mods {
            for diagnostic in &mod_.value().0 {
                output.push(diagnostic.clone());
            }
        }

        output.into_iter()
    }

    pub fn get_mod_diagnostics(&self, path: &ModPath) -> impl Iterator<Item = Diagnostic> {
        self.mods
            .get(path)
            .map(|mod_| mod_.0.iter().cloned().collect::<Vec<_>>())
            .into_iter()
            .flatten()
    }
    pub fn get_mod_highlighting(&self, path: &ModPath) -> impl Iterator<Item = (Span, HighlightColor)> {
        self.mods
            .get(path)
            .map(|mod_| mod_.1.iter().cloned().collect::<Vec<_>>())
            .into_iter()
            .flatten()
    }
}

fn parse(content: &str, interner: Arc<Interner>) -> (Vec<Diagnostic>, Vec<(Span, HighlightColor)>) {
    let src_file = SrcFile::from_str(content);
    let mut context = ParseContext {
        diagnostics: Vec::new(),
        highlighter: Highlighter::new(),
        interner,
    };

    let _ = src_file.tokenize(&mut context).parse_ast().nameres(&mut context);

    (context.diagnostics, context.highlighter.highlights)
}
