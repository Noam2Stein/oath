use crate::*;

#[derive(Debug, Clone, Spanned, InternedDisplay)]
pub enum TokenError {
    #[display("unknown token")]
    UnknownToken(#[span] Span),
    #[display("unclosed {}", field_1.open_str())]
    Unclosed(#[span] Span, DelimiterKind),
    #[display("unopened {}", field_1.close_str())]
    Unopened(#[span] Span, DelimiterKind),
}
