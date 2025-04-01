use crate::*;

#[derive(Debug, Clone, Spanned)]
pub enum TokenError {
    UnknownToken(#[span] Span),
    Unclosed(#[span] Span, DelimiterKind),
    Unopened(#[span] Span, DelimiterKind),
}
