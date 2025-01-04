use oath_src::{Span, SpanLined, Spanned};

use super::Keyword;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    str: String,
    span: SpanLined,
}
impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span.unlined()
    }
}
impl Ident {
    pub fn new(str: String, span: SpanLined) -> Option<Self> {
        if Keyword::is_keyword(&str) {
            None
        } else {
            Some(Self { str, span })
        }
    }
    pub fn new_adjusted(str: String, span: SpanLined) -> Self {
        if Keyword::is_keyword(&str) {
            Self {
                str: format!("@{str}"),
                span,
            }
        } else {
            Self { str, span }
        }
    }
    #[inline(always)]
    pub unsafe fn new_unchecked(str: String, span: SpanLined) -> Self {
        Self { str, span }
    }
    pub fn new_or_keyword(str: &str, span: SpanLined) -> Result<Self, Keyword> {
        if let Some(keyword) = Keyword::from_str(&str, span) {
            Err(keyword)
        } else {
            Ok(Self {
                str: str.to_string(),
                span,
            })
        }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.str
    }
}
