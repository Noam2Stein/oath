use oath_src::{Span, Spanned};

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

    pub fn from_str(span: Span, str: &str, errors: &mut ErrorsHandle) -> Self {
        let mut chars = str.chars().peekable();

        let integral_value_str = {
            let mut value_str = String::with_capacity(str.len());
            if let Some(first_digit) = chars.next() {
                value_str.push(first_digit);
            } else {
                errors.push(Error::new(
                    span,
                    format!("expected int literal, found empty string"),
                ));
            };
            while let Some(maybe_digit) = chars.next() {
                match maybe_digit {
                    digit if maybe_digit.is_ascii_digit() => {
                        value_str.push(digit);
                    }
                    '_' => {}
                    '.' => break,
                    _ => errors.push(Error::new(
                        span,
                        format!("expected either a digit or '.', found '{maybe_digit}'"),
                    )),
                }
            }

            value_str
        };
        let fractional_value_str = {
            let mut value_str = String::with_capacity(str.len() - integral_value_str.len());
            if let Some(first_digit) = chars.next() {
                value_str.push(first_digit);
            } else {
                errors.push(Error::new(
                    span,
                    format!("unexpected end of float literal, expected a number"),
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

            value_str
        };

        let suffix_str = chars.collect::<String>();

        Self {
            span,
            integral_value: u128::from_str_radix(&integral_value_str, 10).unwrap_or_else(|_| {
                errors.push(Error::new(
                    span,
                    format!("'{integral_value_str}' is not a number"),
                ));
                1
            }),
            fractional_value: u128::from_str_radix(&fractional_value_str, 10).unwrap_or_else(
                |_| {
                    errors.push(Error::new(
                        span,
                        format!("'{integral_value_str}' is not a number"),
                    ));
                    1
                },
            ),
            suffix: if suffix_str.is_empty() {
                None
            } else {
                Some(FloatSuffix::try_from_str(&suffix_str).unwrap_or_else(|| {
                    errors.push(Error::new(
                        span,
                        format!("'{suffix_str}' is not an int suffix"),
                    ));
                    FloatSuffix::Float
                }))
            },
        }
    }
}
