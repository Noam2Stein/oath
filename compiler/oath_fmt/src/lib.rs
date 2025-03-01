mod format_tokens;

mod config;
mod formatter;
pub use config::*;
pub use formatter::*;

pub trait CodeFmt {
    fn will_expand(&self, f: &CodeFormatter) -> bool;
}

impl<T: CodeFmt> CodeFmt for &T {
    fn will_expand(&self, f: &CodeFormatter) -> bool {
        self.will_expand(f)
    }
}

impl<T: CodeFmt> CodeFmt for &mut T {
    fn will_expand(&self, f: &CodeFormatter) -> bool {
        self.will_expand(f)
    }
}
