use super::*;

impl Format for Keyword {
    fn format(&self, _interner: &Interner) -> FormatTree {
        FormatTree::AtomStr(self.as_str())
    }
}
with_tokens! {$(
    impl Format for $keyword_type {
        fn format(&self, _interner: &Interner) -> FormatTree {
            FormatTree::AtomStr($keyword)
        }
    }
)*}

impl Format for Ident {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}

impl Format for Punct {
    fn format(&self, _interner: &Interner) -> FormatTree {
        FormatTree::AtomStr(self.as_str())
    }
}
with_tokens! {$(
    impl Format for $punct_type {
        fn format(&self, _interner: &Interner) -> FormatTree {
            FormatTree::AtomStr($punct)
        }
    }
)*}

impl Format for Literal {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
impl Format for IntLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
impl Format for FloatLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
impl Format for CharLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
impl Format for StrLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
