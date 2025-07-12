use super::*;

impl Format for UnOp {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Neg(value) => value.format(interner),
            Self::Not(value) => value.format(interner),

            Self::Eq(value) => value.format(interner),
            Self::NotEq(value) => value.format(interner),
            Self::Less(value) => value.format(interner),
            Self::More(value) => value.format(interner),
            Self::LessEq(value) => value.format(interner),
            Self::MoreEq(value) => value.format(interner),

            Self::Ref(value) => value.format(interner),
            Self::Deref(value) => value.format(interner),
            Self::Lifetime(value) => value.format(interner),

            Self::Question(value) => value.format(interner),
            Self::RangeExclusive(value) => value.format(interner),
            Self::RangeInclusive(value) => value.format(interner),
        }
    }
}

impl Format for BinOp {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Add(value) => value.format(interner),
            Self::Sub(value) => value.format(interner),
            Self::Mul(value) => value.format(interner),
            Self::Div(value) => value.format(interner),
            Self::Rem(value) => value.format(interner),

            Self::And(value) => value.format(interner),
            Self::Or(value) => value.format(interner),
            Self::Xor(value) => value.format(interner),
            Self::Shl(value) => value.format(interner),
            Self::Shr(value) => value.format(interner),

            Self::Bound(value) => value.format(interner),
            Self::RangeExclusive(value) => value.format(interner),
            Self::RangeInclusive(value) => value.format(interner),
        }
    }
}

// Ref

impl Format for Ref {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain([self.punct.format(interner), self.bounds.format(interner)].into())
    }
}

impl Format for RefModifier {
    fn format(&self, interner: &Interner) -> FormatTree {
        let content = match self {
            Self::Lifetime(value) => value.format(interner),
            Self::Mut(value) => value.format(interner),
            Self::Sole(value) => value.format(interner),
            Self::SoleMut(value) => value.format(interner),
        };

        FormatTree::Chain([content, FormatTree::AtomStr(" ")].into())
    }
}

impl Format for Lifetime {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain([self.punct.format(interner), self.ident.format(interner)].into())
    }
}
