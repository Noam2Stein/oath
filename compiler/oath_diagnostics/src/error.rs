use std::fmt::{self, Display, Formatter};

use oath_src::{Span, SpanLengthed, Spanned};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    UnknownToken(Span),
    UnclosedParen(SpanLengthed<1>),
    UnclosedBracket(SpanLengthed<1>),
    UnclosedBrace(SpanLengthed<1>),
    UnopenedParen(SpanLengthed<1>),
    UnopenedBracket(SpanLengthed<1>),
    UnopenedBrace(SpanLengthed<1>),
    StaticMessage(Span, &'static str),
}
impl Spanned for Error {
    fn span(&self) -> Span {
        match self {
            Self::UnknownToken(span) => *span,
            Self::UnclosedParen(span) => span.unlined(),
            Self::UnclosedBracket(span) => span.unlined(),
            Self::UnclosedBrace(span) => span.unlined(),
            Self::UnopenedParen(span) => span.unlined(),
            Self::UnopenedBracket(span) => span.unlined(),
            Self::UnopenedBrace(span) => span.unlined(),
            Self::StaticMessage(span, _) => *span,
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownToken(_) => write!(f, "unknown token"),
            Self::UnclosedParen(_) => write!(f, "unclosed `(`"),
            Self::UnclosedBracket(_) => write!(f, "unclosed `[`"),
            Self::UnclosedBrace(_) => write!(f, "unclosed `{{`"),
            Self::UnopenedParen(_) => write!(f, "unopened `)`"),
            Self::UnopenedBracket(_) => write!(f, "unopened `]`"),
            Self::UnopenedBrace(_) => write!(f, "unopened `}}`"),
            Self::StaticMessage(_, message) => write!(f, "{message}"),
        }
    }
}
