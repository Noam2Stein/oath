use super::*;

impl<T: ToFormatTree> ToFormatTree for Try<T> {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Try::Success(t) => t.to_format_tree(interner),
            Try::Failure(_) => FormatTree::None,
        }
    }
}
