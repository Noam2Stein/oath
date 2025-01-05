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

    pub fn from_str(str: &str, span: SpanLined, diagnostics: DiagnosticsHandle) -> Self {
        let mut chars = str.chars().peekable();

        let mut value_str = String::with_capacity(str.len());
        if let Some(first_digit) = chars.next() {
            value_str.push(first_digit);
        } else {
            diagnostics.push_error(Error::StaticMessage(
                span.unlined(),
                "expected an int literal. found an empty string",
            ));
        };
        while let Some(maybe_digit) = chars.peek() {
            match *maybe_digit {
                digit if maybe_digit.is_ascii_digit() => {
                    value_str.push(digit);
                    chars.next();
                }
                '_' => {
                    chars.next();
                }
                _ => break,
            }
        }

        let suffix_str = chars.collect::<String>();

        Self {
            int: u128::from_str_radix(&value_str, 10).unwrap_or_else(|_| {
                diagnostics.push_error(Error::StaticMessage(span.unlined(), "not a number"));
                1
            }),
            span,
            suffix: if suffix_str.is_empty() {
                None
            } else {
                Ident::new(suffix_str.to_string(), span).map_or_else(
                    || {
                        diagnostics.push_error(Error::StaticMessage(
                            span.unlined(),
                            "expected an ident. found a keyword",
                        ));
                        None
                    },
                    |ident| Some(ident),
                )
            },
        }
    }
}
