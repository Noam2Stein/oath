use oath_diagnostics::{Desc, DiagnosticsHandle, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Literal, LiteralType, TokenDowncastFrom, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrLiteral {
    str: String,
    span: Span,
}

impl LiteralType for StrLiteral {}
impl TokenType for StrLiteral {}
impl Seal for StrLiteral {}
impl Spanned for StrLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}
impl Fill for StrLiteral {
    fn fill(span: Span) -> Self {
        Self::new("_fill_".to_string(), span)
    }
}
impl Desc for StrLiteral {
    fn desc() -> &'static str {
        "an str literal"
    }
}

impl StrLiteral {
    #[inline(always)]
    pub fn new(str: String, span: Span) -> Self {
        Self { str, span }
    }

    #[inline(always)]
    pub fn str(&self) -> &str {
        &self.str
    }

    pub unsafe fn from_regex_str(str: &str, span: Span, _diagnostics: DiagnosticsHandle) -> Self {
        Self {
            str: str[1..str.len() - 1].to_string(),
            span,
        }
    }
}
