use super::*;

mod attrs;
mod expr;
mod items;
mod tokens;

pub trait ToFormatTree {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree;
}

impl<T: ToFormatTree> ToFormatTree for Option<T> {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Some(t) => t.to_format_tree(interner),
            None => FormatTree::None,
        }
    }
}
impl<T: ToFormatTree> ToFormatTree for Try<T> {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Try::Success(t) => t.to_format_tree(interner),
            Try::Failure(_) => FormatTree::TryFailure,
        }
    }
}
