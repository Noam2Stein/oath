use oath_diagnostics::{Desc, DiagnosticsHandle, Error, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Ident, Literal, LiteralType, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatLiteral {
    integral: u128,
    leading_zeros: u128,
    fractional: u128,
    suffix: Option<Ident>,
    span: Span,
}

impl LiteralType for FloatLiteral {}
impl TokenType for FloatLiteral {}
impl Seal for FloatLiteral {}
impl Spanned for FloatLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}
impl Fill for FloatLiteral {
    fn fill(span: Span) -> Self {
        Self::new(1, 0, 0, None, span)
    }
}
impl Desc for FloatLiteral {
    fn desc() -> &'static str {
        "a float literal"
    }
}
impl TryFrom<Literal> for FloatLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Float(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a Literal> for &'a FloatLiteral {
    type Error = ();

    fn try_from(value: &'a Literal) -> Result<Self, Self::Error> {
        if let Literal::Float(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl TryFrom<TokenTree> for FloatLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Float(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for &'a FloatLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Float(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
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
                diagnostics.push_error(Error::StaticMessage("expected `_._`"), span);
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
            diagnostics.push_error(Error::StaticMessage("expected `_._`"), span);
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
            diagnostics.push_error(Error::StaticMessage("out of bounds intergal"), span);
            1
        });

        let leading_zeros = fractional_str
            .chars()
            .position(|char| char != '0')
            .unwrap_or(0) as u128;

        let fractional = u128::from_str_radix(fractional_str, 10).unwrap_or_else(|_| {
            diagnostics.push_error(Error::StaticMessage("out of bounds fractional"), span);
            1
        });

        let suffix = suffix_str.map_or(None, |suffix_str| {
            Ident::new(suffix_str.to_string(), span).or_else(|| {
                diagnostics.push_error(
                    Error::StaticMessage("expected an ident. found a keyword"),
                    span,
                );

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
