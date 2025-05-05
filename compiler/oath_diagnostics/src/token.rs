use crate::*;

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
