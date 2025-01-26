use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::{Span, SpanLined, Spanned};

use crate::Seal;

use super::{Ident, LiteralType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatLiteral {
    integral: u128,
    leading_zeros: u128,
    fractional: u128,
    suffix: Option<Ident>,
    span: Span,
}

impl LiteralType for FloatLiteral {}
impl Seal for FloatLiteral {}
impl Spanned for FloatLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl FloatLiteral {
    #[inline(always)]
    pub fn new(
        integral: u128,
        leading_zeros: u128,
        fractional: u128,
        suffix: Option<Ident>,
        span: Span,
    ) -> Self {
        Self {
            integral,
            leading_zeros,
            fractional,
            suffix,
            span,
        }
    }

    #[inline(always)]
    pub fn integral(self) -> u128 {
        self.integral
    }
    #[inline(always)]
    pub fn leading_zeros(self) -> u128 {
        self.leading_zeros
    }
    #[inline(always)]
    pub fn fractional(self) -> u128 {
        self.fractional
    }
    #[inline(always)]
    pub fn suffix(self) -> Option<Ident> {
        self.suffix
    }

    pub unsafe fn from_regex_str(str: &str, span: Span, diagnostics: DiagnosticsHandle) -> Self {
        let dot_position = str.char_indices().position(|(_, char)| char == '.');
        let dot_position = match dot_position {
            Some(some) => some,
            None => {
                diagnostics.push_error(Error::StaticMessage(span, "expected `_._`"));
                return Self {
                    integral: 1,
                    fractional: 0,
                    leading_zeros: 0,
                    span,
                    suffix: None,
                };
            }
        };

        if str.len() == dot_position + 1 {
            diagnostics.push_error(Error::StaticMessage(span, "expected `_._`"));
            return Self {
                integral: 1,
                fractional: 0,
                leading_zeros: 0,
                span,
                suffix: None,
            };
        };

        let suffix_start = str[dot_position..]
            .char_indices()
            .position(|(_, char)| char.is_alphabetic());

        let intergal_str = &str[0..dot_position];
        let fractional_str = &str[dot_position + 1..suffix_start.unwrap_or(str.len())];
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let integral = u128::from_str_radix(intergal_str, 10).unwrap_or_else(|_| {
            diagnostics.push_error(Error::StaticMessage(span, "out of bounds intergal"));
            1
        });

        let leading_zeros = fractional_str
            .chars()
            .position(|char| char != '0')
            .unwrap_or(0) as u128;

        let fractional = u128::from_str_radix(fractional_str, 10).unwrap_or_else(|_| {
            diagnostics.push_error(Error::StaticMessage(span, "out of bounds fractional"));
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

        Self {
            integral,
            leading_zeros,
            fractional,
            span,
            suffix,
        }
    }
}
