use oath_diagnostics::DiagnosticsHandle;
use oath_src::{Span, SpanLengthed, Spanned};

use crate::Seal;

use super::LiteralType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharLiteral {
    char: char,
    span: SpanLengthed<3>,
}

impl LiteralType for CharLiteral {}
impl Seal for CharLiteral {}
impl Spanned for CharLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span.unlined()
    }
}

impl CharLiteral {
    #[inline(always)]
    pub fn new(char: char, span: SpanLengthed<3>) -> Self {
        Self { char, span }
    }

    #[inline(always)]
    pub fn char(self) -> char {
        self.char
    }

    pub unsafe fn from_regex_str(
        str: &str,
        span: SpanLengthed<3>,
        _diagnostics: DiagnosticsHandle,
    ) -> Self {
        Self {
            char: str.chars().skip(1).next().unwrap(),
            span,
        }
    }
}
