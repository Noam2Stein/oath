use derive_more::Display;

use crate::*;

#[derive(Debug, Clone, Copy, Hash, Spanned)]
pub enum SyntaxError {
    Expected(#[span] Span, &'static str),
    CannotBeMarked(#[span] Span, &'static str, &'static str),
    CannotHaveATarget(#[span] Span, &'static str),
    Mutliple(#[span] Span, &'static str),
}

#[derive(Debug, Clone, Copy, Hash, Spanned)]
pub enum SyntaxWarning {
    UnnesessaryParens(#[span] Parens),
    ExpectedCase(#[span] Ident, IdentCase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum IdentCase {
    #[display("UpperCamelCase")]
    UpperCamelCase,
    #[display("lowerCamelCase")]
    LowerCamelCase,
}
