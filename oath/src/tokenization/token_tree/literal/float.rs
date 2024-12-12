use super::*;
#[derive(Debug, Clone, Copy, Hash)]
pub struct FloatLiteral {
    span: Span,
    integral_value: u128,
    fractional_value: u128,
    suffix: Option<FloatSuffix>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FloatSuffix {
    Float,
    Float16,
    Float64,
    Float128,
}
impl Display for FloatLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.integral_value, self.fractional_value)?;

        if let Some(suffix) = self.suffix {
            write!(f, "{suffix}")?;
        }

        Ok(())
    }
}
impl Display for FloatSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float => "float".fmt(f),
            Self::Float16 => "float16".fmt(f),
            Self::Float64 => "float64".fmt(f),
            Self::Float128 => "float128".fmt(f),
        }
    }
}
impl Spanned for FloatLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl FloatLiteral {
    pub fn new(
        span: Span,
        integral_value: u128,
        fractional_value: u128,
        suffix: Option<FloatSuffix>,
    ) -> Self {
        Self {
            integral_value,
            fractional_value,
            suffix,
            span,
        }
    }

    pub fn unsuffixed(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self {
            span,
            integral_value,
            fractional_value,
            suffix: None,
        }
    }
    pub fn float16(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self {
            span,
            integral_value,
            fractional_value,
            suffix: Some(FloatSuffix::Float16),
        }
    }
    pub fn float32(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self {
            span,
            integral_value,
            fractional_value,
            suffix: Some(FloatSuffix::Float),
        }
    }
    pub fn float64(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self {
            span,
            integral_value,
            fractional_value,
            suffix: Some(FloatSuffix::Float64),
        }
    }
    pub fn float128(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self {
            span,
            integral_value,
            fractional_value,
            suffix: Some(FloatSuffix::Float128),
        }
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
impl FloatSuffix {
    pub fn try_from_str(str: &str) -> Option<Self> {
        match str {
            "float" => Some(Self::Float),
            "float16" => Some(Self::Float16),
            "float64" => Some(Self::Float64),
            "float128" => Some(Self::Float128),
            _ => None,
        }
    }
}

impl Literal {
    pub fn float(
        span: Span,
        integral_value: u128,
        fractional_value: u128,
        suffix: Option<FloatSuffix>,
    ) -> Self {
        Self::Float(FloatLiteral::new(
            span,
            integral_value,
            fractional_value,
            suffix,
        ))
    }

    pub fn float_unsuffixed(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self::Float(FloatLiteral::unsuffixed(
            span,
            integral_value,
            fractional_value,
        ))
    }
    pub fn float16(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self::Float(FloatLiteral::float16(
            span,
            integral_value,
            fractional_value,
        ))
    }
    pub fn float32(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self::Float(FloatLiteral::float32(
            span,
            integral_value,
            fractional_value,
        ))
    }
    pub fn float64(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self::Float(FloatLiteral::float64(
            span,
            integral_value,
            fractional_value,
        ))
    }
    pub fn float128(span: Span, integral_value: u128, fractional_value: u128) -> Self {
        Self::Float(FloatLiteral::float128(
            span,
            integral_value,
            fractional_value,
        ))
    }
}
