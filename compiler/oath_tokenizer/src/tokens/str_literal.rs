use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrLiteral {
    pub str_id: StrId,
    span: Span,
}

impl LiteralType for StrLiteral {}
impl TokenType for StrLiteral {}
impl Seal for StrLiteral {}

impl TryFrom<TokenTree> for StrLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Str(value)) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for StrLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Str(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}
impl TryFrom<Literal> for StrLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Str(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl Spanned for StrLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl StrLiteral {
    #[inline(always)]
    pub fn new(str_id: StrId, span: Span) -> Self {
        Self { str_id, span }
    }

    pub unsafe fn from_regex_str(str: &str, span: Span, context: ContextHandle) -> Self {
        Self {
            str_id: context.intern(&str[1..str.len() - 1]),
            span,
        }
    }
}
