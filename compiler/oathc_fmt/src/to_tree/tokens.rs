use super::*;

impl ToFormatTree for Keyword {
    fn to_format_tree(&self, _interner: &Interner) -> FormatTree {
        FormatTree::Atom(self.to_string())
    }
}
with_tokens! {$(
    impl ToFormatTree for $keyword_type {
        fn to_format_tree(&self, _interner: &Interner) -> FormatTree {
            FormatTree::Atom(self.to_string())
        }
    }
)*}

impl ToFormatTree for Ident {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Atom(interner.unintern(self.str_id()))
    }
}

impl ToFormatTree for Punct {
    fn to_format_tree(&self, _interner: &Interner) -> FormatTree {
        FormatTree::Atom(self.to_string())
    }
}
with_tokens! {$(
    impl ToFormatTree for $punct_type {
        fn to_format_tree(&self, _interner: &Interner) -> FormatTree {
            FormatTree::Atom(self.to_string())
        }
    }
)*}

impl ToFormatTree for Literal {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        match self {
            Literal::Int(lit) => lit.to_format_tree(interner),
            Literal::Float(lit) => lit.to_format_tree(interner),
            Literal::Str(lit) => lit.to_format_tree(interner),
            Literal::Char(lit) => lit.to_format_tree(interner),
        }
    }
}
impl ToFormatTree for IntLiteral {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Atom(self.to_string_interned(interner))
    }
}
impl ToFormatTree for FloatLiteral {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Atom(self.to_string_interned(interner))
    }
}
impl ToFormatTree for CharLiteral {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Atom(self.to_string_interned(interner))
    }
}
impl ToFormatTree for StrLiteral {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Atom(self.to_string_interned(interner))
    }
}
