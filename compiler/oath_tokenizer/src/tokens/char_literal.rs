use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharLiteral {
    char: char,
    span: Span,
}

impl LiteralType for CharLiteral {}
impl TokenType for CharLiteral {}
impl Seal for CharLiteral {}

impl TryFrom<TokenTree> for CharLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Char(value)) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for CharLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Char(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}
impl TryFrom<Literal> for CharLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Char(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl Spanned for CharLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
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

    pub unsafe fn from_regex_str(str: &str, span: Span, _context: ContextHandle) -> Self {
        Self {
            char: str.chars().skip(1).next().unwrap(),
            span,
        }
    }
}
