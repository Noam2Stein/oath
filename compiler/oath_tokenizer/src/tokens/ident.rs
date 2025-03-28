use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    str_id: StrId,
    span: Span,
}

impl TokenType for Ident {}
impl Seal for Ident {}

impl TryFrom<TokenTree> for Ident {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Ident(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for Ident {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Ident(value) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl Ident {
    pub fn new(str: &str, span: Span, context: ContextHandle) -> Option<Self> {
        if is_keyword(&str) {
            None
        } else {
            Some(Self {
                str_id: context.intern(str),
                span,
            })
        }
    }
    pub fn new_adjusted(str: &str, span: Span, context: ContextHandle) -> Self {
        if is_keyword(&str) {
            Self {
                str_id: context.intern(&format!("@{str}")),
                span,
            }
        } else {
            Self {
                str_id: context.intern(str),
                span,
            }
        }
    }
    #[inline(always)]
    pub unsafe fn new_unchecked(str: &str, span: Span, context: ContextHandle) -> Self {
        Self {
            str_id: context.intern(str),
            span,
        }
    }
    pub fn new_or_keyword(str: &str, span: Span, context: ContextHandle) -> Result<Self, Keyword> {
        if let Some(keyword) = Keyword::from_str(&str, span) {
            Err(keyword)
        } else {
            Ok(Self {
                str_id: context.intern(str),
                span,
            })
        }
    }

    #[inline(always)]
    pub fn str_id(&self) -> StrId {
        self.str_id
    }
}

impl Highlightable for Ident {
    fn highlight_span(&self) -> Option<Span> {
        Some(self.span)
    }
}
