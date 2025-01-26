use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::{Span, SpanLined, Spanned};

use crate::Seal;

use super::{Ident, LiteralType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntLiteral {
    int: u128,
    suffix: Option<Ident>,
    span: SpanLined,
}

impl LiteralType for IntLiteral {}
impl Seal for IntLiteral {}
impl Spanned for IntLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span.unlined()
    }
}

impl IntLiteral {
    #[inline(always)]
    pub fn new(int: u128, suffix: Option<Ident>, span: SpanLined) -> Self {
        Self { int, suffix, span }
    }

    #[inline(always)]
    pub fn int(self) -> u128 {
        self.int
    }
    #[inline(always)]
    pub fn suffix(self) -> Option<Ident> {
        self.suffix
    }

    pub unsafe fn from_regex_str(
        str: &str,
        span: SpanLined,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let suffix_start = str
            .char_indices()
            .find(|(_, char)| !char.is_ascii_digit() && *char != '_')
            .map(|(char_pos, _)| char_pos);

        let int_str = &str[0..suffix_start.unwrap_or(str.len())].replace("_", "");
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let int = u128::from_str_radix(int_str, 10).unwrap_or_else(|_| {
            diagnostics.push_error(Error::StaticMessage(
                span.unlined(),
                "out of bounds literal",
            ));
            1
        });

        let suffix = suffix_str.map_or(None, |suffix_str| {
            let span = SpanLined::from_end_len(span.end(), suffix_str.len() as _);
            Ident::new(suffix_str.to_string(), span).or_else(|| {
                diagnostics.push_error(Error::StaticMessage(
                    span.unlined(),
                    "expected an ident. found a keyword",
                ));
                None
            })
        });

        Self { int, suffix, span }
    }
}
