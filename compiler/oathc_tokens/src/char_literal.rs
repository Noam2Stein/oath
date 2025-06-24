use super::*;

#[derive(Debug, InternedDisplay, new, Spanned)]
#[display("{char:?}")]
pub struct CharLiteral {
    #[span]
    pub span: Span,
    pub char: char,
}

const _: () = verify_token_type::<CharLiteral>();
