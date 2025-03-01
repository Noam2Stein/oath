use std::fmt::Formatter;

use oath_tokenizer::DelimiterKind;

use crate::{CodeFmt, CodeFmtConfig};

pub struct CodeFormatter<'a> {
    config: &'a CodeFmtConfig,
    line: u32,
    indent_level: u32,
    formatter: Formatter<'a>,
}
impl<'a> CodeFormatter<'a> {
    pub fn new(config: &'a CodeFmtConfig, formatter: Formatter<'a>) -> Self {
        Self {
            config,
            line: 0,
            indent_level: 0,
            formatter,
        }
    }

    pub fn newline(&mut self) {
        self.string.push('\n');
        for _ in 0..self.indent_level {
            self.string.push_str(&self.indent);
        }
    }
    pub fn space(&mut self) {
        self.string.push(' ');
    }
    pub fn in_indent(&mut self, delims: DelimiterKind, write: impl FnOnce(&mut Self)) {
        self.string += delims.open_str();

        self.indent_level += 1;
        self.newline();

        write(self);

        self.newline();
        self.indent_level -= 1;

        self.string += delims.close_str();
    }
    pub fn format(&mut self, value: impl CodeFmt) {}
    pub fn list(&mut self, values: impl Iterator<Item = impl CodeFmt>) {
        if self.config.propagate_multiline {
            let values = values.collect::<Vec<_>>();
            if values.iter().any(|value| value.will_expand(self)) {}
        } else {
        }
    }
}
