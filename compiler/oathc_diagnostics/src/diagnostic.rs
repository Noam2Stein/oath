use derive_more::{Display, From, TryInto};

use super::*;

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Diagnostic {
    Error(Error),
    Warning(Warning),
}

#[derive(Debug, Clone, TryInto, Spanned, InternedDisplay)]
pub enum Error {
    #[display("unknown token")]
    UnknownToken(#[span] Span),
    #[display("unclosed `{field_1}`")]
    UnclosedDelimiter(#[span] Span, &'static str),
    #[display("unopened `{field_1}`")]
    UnopenedDelimiter(#[span] Span, &'static str),
    #[display("out of bounds literal")]
    OutOfBoundsLiteral(#[span] Span),

    #[display("Syntax: expected {}", field_1)]
    Expected(#[span] Span, &'static str),
    #[display("Syntax: unexpected tokens")]
    UnexpectedTokens(#[span] Span),

    #[display("found both `{field_1}.oh` and `{field_1}/mod.oh`")]
    DoubleMod(#[span] Span, StrId),
    #[display("cannot find module `{field_1}`")]
    NoMod(#[span] Span, StrId),
    #[display("invalid file")]
    InvalidFile(#[span] Span),

    #[display("Syntax: {field_1} cannot be marked {field_2}")]
    CannotBeMarked(#[span] Span, &'static str, &'static str),
    #[display("Syntax: {} cannot cannot have a target item kind", field_1)]
    CannotHaveATarget(#[span] Span, &'static str),
    #[display("Syntax: multiple {}", field_1)]
    Mutliple(#[span] Span, &'static str),
    #[display("uninit variables are not allowed")]
    UninitVariable(#[span] Span),
    #[display("`{field_1}` doesn't exist in this context")]
    DoesntExist(#[span] Span, StrId),
    #[display("`{field_1}` already exists in this context")]
    AlreadyExists(#[span] Span, StrId),
    #[display("todo")]
    ToDo(Span),
}

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Warning {
    #[display("Syntax: unnesessary parens")]
    UnnesessaryParens(#[span] Span),
    #[display("Syntax: expected {}", field_1)]
    ExpectedCase(#[span] Span, IdentCase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum IdentCase {
    #[display("UpperCamelCase")]
    UpperCamelCase,
    #[display("lowerCamelCase")]
    LowerCamelCase,
}
