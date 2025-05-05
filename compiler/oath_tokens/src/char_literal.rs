use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new, Spanned)]
#[display("{char:?}")]
pub struct CharLiteral {
    #[span]
    pub span: Span,
    pub char: char,
}

verify_token_type!(CharLiteral);
