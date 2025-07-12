use super::*;

impl Format for Expr {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::SpacedChain(FormatSpacing::SpaceOrLine, {
            self.attrs
                .iter()
                .map(|attr| attr.format(interner))
                .chain([FormatTree::SpacedChain(
                    FormatSpacing::SpaceOrLineTab,
                    [self.first_unary.format(interner)]
                        .into_iter()
                        .chain(self.bin_op_exts.iter().map(|bin_op_ext| bin_op_ext.format(interner)))
                        .collect(),
                )])
                .collect()
        })
    }
}

impl Format for UnaryExpr {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::SpacedChain(
            FormatSpacing::SpaceOrLineTab,
            [FormatTree::Chain(
                self.prefixes
                    .iter()
                    .map(|prefix| prefix.format(interner))
                    .chain([self.core.format(interner)])
                    .collect(),
            )]
            .into_iter()
            .chain(self.exts.iter().map(|ext| ext.format(interner)))
            .collect(),
        )
    }
}

impl Format for ExprCore {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Array(value) => value.format(interner),
            Self::Block(value) => value.format(interner),
            Self::For(value) => value.format(interner),
            Self::Ident(value) => value.format(interner),
            Self::If(value) => value.format(interner),
            Self::Keyword(value) => value.format(interner),
            Self::Literal(value) => value.format(interner),
            Self::Loop(value) => value.format(interner),
            Self::Tuple(value) => value.format(interner),
            Self::Until(value) => value.format(interner),
            Self::While(value) => value.format(interner),
        }
    }
}

impl Format for ExprBinOpExt {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain([self.op.format(interner), self.rhs.format(interner)].into())
    }
}

impl Format for UnaryExprExt {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Call(value) => value.format(interner),
            Self::Construct(value) => value.format(interner),
            Self::Generics(value) => value.format(interner),
            Self::Index(value) => value.format(interner),
            Self::Member(value) => value.format(interner),
        }
    }
}

impl Format for Tuple {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::DenseDelims(
            "(",
            Box::new(FormatTree::SpacedChain(
                FormatSpacing::Colon,
                self.items.iter().map(|item| item.format(interner)).collect(),
            )),
            self.frame.leftovers.text.clone(),
            ")",
        )
    }
}

impl Format for Array {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::DenseDelims(
            "[",
            Box::new(FormatTree::SpacedChain(
                FormatSpacing::Colon,
                self.items.iter().map(|item| item.format(interner)).collect(),
            )),
            self.frame.leftovers.text.clone(),
            "]",
        )
    }
}

impl Format for Assign {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::SpacedChain(
            FormatSpacing::Space,
            [self.eq.format(interner), self.value.format(interner)].into(),
        )
    }
}
