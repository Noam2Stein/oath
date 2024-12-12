use super::*;

mod char;
mod float;
mod int;
mod string;
pub use char::*;
pub use float::*;
pub use int::*;
pub use string::*;

#[derive(Debug, Clone, Hash)]
pub enum Literal {
    String(StringLiteral),
    Char(CharLiteral),
    Int(IntLiteral),
    Float(FloatLiteral),
    Error(Span),
}
impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(literal) => literal.fmt(f),
            Self::Char(literal) => literal.fmt(f),
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
            Self::Error(_) => "".fmt(f),
        }
    }
}
impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Self::String(literal) => literal.span(),
            Self::Char(literal) => literal.span(),
            Self::Int(literal) => literal.span(),
            Self::Float(literal) => literal.span(),
            Self::Error(span) => *span,
        }
    }
}
