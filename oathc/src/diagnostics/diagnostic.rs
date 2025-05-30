use derive_more::{Display, From, TryInto};

use super::*;

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Diagnostic {
    Error(Error),
    Warning(Warning),
}

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Error {
    Token(TokenError),
    Syntax(SyntaxError),
    Name(NameError),
    #[display("unfinished")]
    Unfinished(Span),
}

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Warning {
    Syntax(SyntaxWarning),
}

#[derive(Debug, Clone, Spanned, InternedDisplay)]
pub enum TokenError {
    #[display("unknown token")]
    UnknownToken(#[span] Span),
    #[display("unclosed {field_0}")]
    Unclosed(OpenDelimiter),
    #[display("unopened {field_0}")]
    Unopened(CloseDelimiter),
    #[display("out of bounds literal")]
    OutOfBoundsLiteral(#[span] Span),
}

#[derive(Debug, Clone, Hash, Spanned, InternedDisplay)]
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

#[derive(Debug, Clone, Hash, Spanned, InternedDisplay)]
pub enum NameError {
    #[display("`{field_0}` doesn't exist in this context")]
    DoesntExist(Ident),
    #[display("`{field_0}` already exists in this context")]
    AlreadyExists(Ident),
}

impl Diagnostic {
    pub fn is_live(&self) -> bool {
        match self {
            Self::Error(Error::Name(_)) => false,
            Self::Error(Error::Syntax(_)) => true,
            Self::Error(Error::Token(_)) => true,
            Self::Error(Error::Unfinished(_)) => false,
            Self::Warning(Warning::Syntax(_)) => true,
        }
    }
}
