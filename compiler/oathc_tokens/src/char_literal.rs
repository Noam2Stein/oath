use super::*;

#[derive(Debug, InternedDisplay, new, Spanned)]
#[display("{char:?}")]
pub struct CharLiteral {
    #[span]
    pub span: Span,
    pub char: char,
}

const _: () = verify_token_type::<CharLiteral>();

impl Format for CharLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
