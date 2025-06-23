use super::*;

impl ToFormatTree for Expr {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::LineChain(
            [self.attrs.to_format_tree(interner), self.first_unary.to_format_tree(interner)]
                .into_iter()
                .chain(self.bin_op_exts.iter().map(|ext| {
                    FormatTree::Chain(
                        [
                            ext.op.to_format_tree(interner),
                            FormatTree::AtomStr(" "),
                            ext.rhs.to_format_tree(interner),
                        ]
                        .into(),
                    )
                }))
                .collect(),
        )
    }
}

impl ToFormatTree for UnaryExpr {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain(
            [FormatTree::Chain(
                self.prefixes
                    .iter()
                    .map(|prefix| prefix.to_format_tree(interner))
                    .chain([self.core.to_format_tree(interner)])
                    .collect(),
            )]
            .into(),
        )
    }
}

impl ToFormatTree for ExprCore {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Array(array) => FormatTree::DenseDelims(
                '[',
                Box::new(FormatTree::List(
                    array.items.iter().map(|item| item.to_format_tree(interner)).collect(),
                )),
                ']',
            ),
            Self::Block(_) => FormatTree::None,
            Self::For(_) => FormatTree::None,
            Self::Loop(_) => FormatTree::None,
            Self::While(_) => FormatTree::None,
            Self::Until(_) => FormatTree::None,
            Self::If(_) => FormatTree::None,
            Self::Ident(value) => value.to_format_tree(interner),
            Self::Keyword(value) => value.to_format_tree(interner),
            Self::Literal(value) => value.to_format_tree(interner),
            Self::Tuple(tuple) => FormatTree::DenseDelims(
                '(',
                Box::new(FormatTree::List(
                    tuple.items.iter().map(|item| item.to_format_tree(interner)).collect(),
                )),
                ')',
            ),
        }
    }
}

impl ToFormatTree for ExprKeyword {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Fn(keyword) => keyword.to_format_tree(interner),
            Self::Out(keyword) => keyword.to_format_tree(interner),
            Self::Type(keyword) => keyword.to_format_tree(interner),
        }
    }
}

impl ToFormatTree for UnaryExprPrefix {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Deref(_) => FormatTree::AtomStr("*"),
            Self::Eq(_) => FormatTree::AtomStr("== "),
            Self::Less(_) => FormatTree::AtomStr("< "),
            Self::LessEq(_) => FormatTree::AtomStr("<= "),
            Self::Lifetime(lifetime) => {
                FormatTree::Chain([FormatTree::AtomStr("'"), lifetime.ident.to_format_tree(interner)].into())
            }
            Self::More(_) => FormatTree::AtomStr("> "),
            Self::MoreEq(_) => FormatTree::AtomStr(">= "),
            Self::Neg(_) => FormatTree::AtomStr("-"),
            Self::Not(_) => FormatTree::AtomStr("!"),
            Self::NotEq(_) => FormatTree::AtomStr("!= "),
            Self::Question(_) => FormatTree::AtomStr("?"),
            Self::Ref(ref_) => FormatTree::Chain([FormatTree::AtomStr("&"), ref_.bounds.to_format_tree(interner)].into()),
        }
    }
}

impl ToFormatTree for RefModifier {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Lifetime(lifetime) => {
                FormatTree::Chain([FormatTree::AtomStr("'"), lifetime.ident.to_format_tree(interner)].into())
            }
            Self::Mut(_) => FormatTree::AtomStr("mut "),
            Self::Sole(_) => FormatTree::AtomStr("sole "),
            Self::SoleMut(_) => FormatTree::AtomStr("smut "),
        }
    }
}

impl ToFormatTree for BinOp {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
