use oath_context::Error;
use oath_src::Span;

use crate::DelimiterKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenError {
    UnknownToken(Span),
    Unclosed(Span, DelimiterKind),
    Unopened(Span, DelimiterKind),
}

impl From<TokenError> for Error {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::UnknownToken(span) => Self::new(format!("unknown token"), span),
            TokenError::Unclosed(span, kind) => {
                Self::new(format!("unclosed `{}`", kind.open_str()), span)
            }
            TokenError::Unopened(span, kind) => {
                Self::new(format!("unopened `{}`", kind.close_str()), span)
            }
        }
    }
}
