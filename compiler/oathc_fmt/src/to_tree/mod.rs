use super::*;

mod attrs;
mod diagnostics;
mod expr;
mod items;
mod tokens;

pub trait ToFormatTree {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree;
}
