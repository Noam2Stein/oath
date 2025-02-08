use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    Token(TokenError),
    Syntax(SyntaxError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token(error) => write!(f, "Token Error: {error}"),
            Self::Syntax(error) => write!(f, "Syntax Error: {error}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxError {
    Expected(&'static str),
    CannotBePutOn(&'static str, &'static str),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected(expected) => write!(f, "expected {expected}"),
            Self::CannotBePutOn(a, b) => write!(f, "{a} cannot be put on {b}"),
        }
    }
}
