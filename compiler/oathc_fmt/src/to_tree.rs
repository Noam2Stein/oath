use super::*;

pub trait ToFormatTree {
    fn to_format_tree(&self) -> FormatTree;
}

impl ToFormatTree for SyntaxTree {
    fn to_format_tree(&self) -> FormatTree {
        FormatTree::Atom(format!("PLACE HOLDER"))
    }
}
