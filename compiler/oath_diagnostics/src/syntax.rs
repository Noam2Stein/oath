use derive_more::Display;

use crate::*;

#[derive(Debug, Clone, Copy, Hash, Spanned, InternedDisplay)]
pub enum SyntaxError {
    #[display("Syntax: expected {}", field_1)]
    Expected(#[span] Span, &'static str),
    #[display("Syntax: unexpected tokens")]
    UnexpectedTokens(#[span] Span),
    #[display("Syntax: {} cannot be marked {}", field_1, field_2)]
    CannotBeMarked(#[span] Span, &'static str, &'static str),
    #[display("Syntax: {} cannot cannot have a target item kind", field_1)]
    CannotHaveATarget(#[span] Span, &'static str),
    #[display("Syntax: multiple {}", field_1)]
    Mutliple(#[span] Span, &'static str),
    #[display("uninit variables are not allowed")]
    UninitVariable(#[span] Span),
}

#[derive(Debug, Clone, Copy, Hash, Spanned, InternedDisplay)]
pub enum SyntaxWarning {
    #[display("Syntax: unnesessary parens")]
    UnnesessaryParens(#[span] Parens),
    #[display("Syntax: expected {}", field_1)]
    ExpectedCase(#[span] Ident, IdentCase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum IdentCase {
    #[display("UpperCamelCase")]
    UpperCamelCase,
    #[display("lowerCamelCase")]
    LowerCamelCase,
}
