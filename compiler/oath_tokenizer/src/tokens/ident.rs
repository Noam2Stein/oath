use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Keyword, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    str: String,
    span: Span,
}

impl TokenType for Ident {}
impl Seal for Ident {}
impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}
impl Fill for Ident {
    fn fill(span: Span) -> Self {
        Self::new_adjusted("_fill_".to_string(), span)
    }
}
impl Desc for Ident {
    fn desc() -> &'static str {
        "an ident"
    }
}
impl TryFrom<TokenTree> for Ident {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Ident(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for &'a Ident {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Ident(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}

impl Ident {
    pub fn new(str: String, span: Span) -> Option<Self> {
        if Keyword::is_keyword(&str) {
            None
        } else {
            Some(Self { str, span })
        }
    }
    pub fn new_adjusted(str: String, span: Span) -> Self {
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
    pub unsafe fn new_unchecked(str: String, span: Span) -> Self {
        Self { str, span }
    }
    pub fn new_or_keyword(str: &str, span: Span) -> Result<Self, Keyword> {
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
