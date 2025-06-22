use super::*;

impl ToFormatTree for SyntaxTree {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::SpacedLineChain(self.items.values.iter().map(|item| item.to_format_tree(interner)).collect())
    }
}

impl ToFormatTree for Item {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        self.
    }
}
