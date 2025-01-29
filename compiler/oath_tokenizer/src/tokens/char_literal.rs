use oath_diagnostics::{Desc, DiagnosticsHandle, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Literal, LiteralType, TokenTree, TokenType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharLiteral {
    char: char,
    span: Span,
}

impl LiteralType for CharLiteral {}
impl TokenType for CharLiteral {}
impl Seal for CharLiteral {}
impl Spanned for CharLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}
impl Fill for CharLiteral {
    fn fill(span: Span) -> Self {
        Self::new('?', span)
    }
}
impl Desc for CharLiteral {
    fn desc() -> &'static str {
        "a char literal"
    }
}
impl TryFrom<Literal> for CharLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Char(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a Literal> for &'a CharLiteral {
    type Error = ();

    fn try_from(value: &'a Literal) -> Result<Self, Self::Error> {
        if let Literal::Char(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl TryFrom<TokenTree> for CharLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Char(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for &'a CharLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Char(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}

impl CharLiteral {
    #[inline(always)]
    pub fn new(char: char, span: Span) -> Self {
        Self { char, span }
    }

    #[inline(always)]
    pub fn char(self) -> char {
        self.char
    }

    pub unsafe fn from_regex_str(str: &str, span: Span, _diagnostics: DiagnosticsHandle) -> Self {
        Self {
            char: str.chars().skip(1).next().unwrap(),
            span,
        }
    }
}
