use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    UnknownToken,
    UnclosedParen,
    UnclosedBracket,
    UnclosedBrace,
    UnclosedAngle,
    UnopenedParen,
    UnopenedBracket,
    UnopenedBrace,
    UnopenedAngle,
    Expected(&'static str),
    StaticMessage(&'static str),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownToken => write!(f, "unknown token"),
            Self::UnclosedParen => write!(f, "unclosed `(`"),
            Self::UnclosedBracket => write!(f, "unclosed `[`"),
            Self::UnclosedBrace => write!(f, "unclosed `{{`"),
            Self::UnclosedAngle => write!(f, "unclosed `<#`"),
            Self::UnopenedParen => write!(f, "unopened `)`"),
            Self::UnopenedBracket => write!(f, "unopened `]`"),
            Self::UnopenedBrace => write!(f, "unopened `}}`"),
            Self::UnopenedAngle => write!(f, "unopened `#>`"),
            Self::Expected(ty) => write!(f, "expected {ty}"),
            Self::StaticMessage(message) => write!(f, "{message}"),
        }
    }
}
