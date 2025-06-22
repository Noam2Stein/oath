use super::*;

impl ToFormatTree for SyntaxTree {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::SpacedLineChain(self.items.iter().map(|item| item.to_format_tree(interner)).collect())
    }
}

impl ToFormatTree for Item {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::LineChain(
            self.attrs
                .iter()
                .map(|attr| attr.to_format_tree(interner))
                .into_iter()
                .chain([FormatTree::SpacedChain(
                    self.modifiers
                        .iter()
                        .map(|modifier| modifier.to_format_tree(interner))
                        .chain([self.core.to_format_tree(interner)])
                        .collect(),
                )])
                .collect(),
        )
    }
}

impl ToFormatTree for ItemModifier {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Comptime(keyword) => keyword.to_format_tree(interner),
            Self::Con(keyword) => keyword.to_format_tree(interner),
            Self::Open(keyword) => keyword.to_format_tree(interner),
            Self::Pub(keyword) => keyword.to_format_tree(interner),
            Self::Raw(keyword) => keyword.to_format_tree(interner),
            Self::Runtime(keyword) => keyword.to_format_tree(interner),
        }
    }
}

impl ToFormatTree for ItemCore {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomStr("struct Vec2<T> { x T, y T }")
    }
}
