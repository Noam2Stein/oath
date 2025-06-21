use oathc_diagnostics::Diagnostics;

use super::*;

pub trait FormatExt {
    fn format(&self, config: &FormatConfig, interner: &Interner, file_interner: &FileInterner) -> String;
}

impl<T: AsRef<str>> FormatExt for T {
    fn format(&self, config: &FormatConfig, interner: &Interner, file_interner: &FileInterner) -> String {
        let text = self.as_ref();

        let path = file_interner.intern("");
        let diagnostics = &Diagnostics::new();
        let highlights = &mut vec![];

        let ast = text.tokenize(path, interner, diagnostics, highlights).parse_ast();
        let tree = ast.to_format_tree();

        let mut s = String::with_capacity(text.len());
        tree.format(&mut s, 0, config).unwrap();

        s
    }
}
