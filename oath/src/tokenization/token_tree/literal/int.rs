use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct IntLiteral {
    span: Span,
    value: u128,
    suffix: Option<IntSuffix>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntSuffix {
    Int,
    Int8,
    Int16,
    Int64,
    Int128,
    IntPtr,
    UInt,
    UInt8,
    UInt16,
    UInt64,
    UInt128,
    UIntPtr,
}
impl Display for IntLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;

        if let Some(suffix) = self.suffix {
            write!(f, "{suffix}")?;
        }

        Ok(())
    }
}
impl Display for IntSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => "int".fmt(f),
            Self::Int8 => "int8".fmt(f),
            Self::Int16 => "int16".fmt(f),
            Self::Int64 => "int64".fmt(f),
            Self::Int128 => "int128".fmt(f),
            Self::IntPtr => "intp".fmt(f),
            Self::UInt => "uint".fmt(f),
            Self::UInt8 => "uint8".fmt(f),
            Self::UInt16 => "uint16".fmt(f),
            Self::UInt64 => "uint64".fmt(f),
            Self::UInt128 => "uint128".fmt(f),
            Self::UIntPtr => "uintp".fmt(f),
        }
    }
}
impl Spanned for IntLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl IntLiteral {
    pub fn new(span: Span, value: u128, suffix: Option<IntSuffix>) -> Self {
        Self {
            span,
            value,
            suffix,
        }
    }

    pub fn unsuffixed(span: Span, value: u128) -> Self {
        Self {
            span,
            value,
            suffix: None,
        }
    }
    pub fn int8(span: Span, value: u8) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::Int8),
        }
    }
    pub fn int16(span: Span, value: u16) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::Int16),
        }
    }
    pub fn int32(span: Span, value: u32) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::Int),
        }
    }
    pub fn int64(span: Span, value: u64) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::Int64),
        }
    }
    pub fn int128(span: Span, value: u128) -> Self {
        Self {
            span,
            value,
            suffix: Some(IntSuffix::Int128),
        }
    }
    pub fn intp(span: Span, value: u128) -> Self {
        Self {
            span,
            value,
            suffix: Some(IntSuffix::IntPtr),
        }
    }
    pub fn uint8(span: Span, value: u8) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::UInt8),
        }
    }
    pub fn uint16(span: Span, value: u16) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::UInt16),
        }
    }
    pub fn uint32(span: Span, value: u32) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::UInt),
        }
    }
    pub fn uint64(span: Span, value: u64) -> Self {
        Self {
            span,
            value: value as u128,
            suffix: Some(IntSuffix::UInt64),
        }
    }
    pub fn uint128(span: Span, value: u128) -> Self {
        Self {
            span,
            value,
            suffix: Some(IntSuffix::UInt128),
        }
    }
    pub fn uintp(span: Span, value: u128) -> Self {
        Self {
            span,
            value,
            suffix: Some(IntSuffix::UIntPtr),
        }
    }

    pub fn from_str(span: Span, str: &str, errors: &mut ErrorsHandle) -> Self {
        let mut chars = str.chars().peekable();

        let mut value_str = String::with_capacity(str.len());
        if let Some(first_digit) = chars.next() {
            value_str.push(first_digit);
        } else {
            errors.push(Error::new(
                span,
                format!("expected int literal, found empty string"),
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
            span,
            value: u128::from_str_radix(&value_str, 10).unwrap_or_else(|_| {
                errors.push(Error::new(span, format!("'{value_str}' is not a number")));
                1
            }),
            suffix: if suffix_str.is_empty() {
                None
            } else {
                Some(IntSuffix::try_from_str(&suffix_str).unwrap_or_else(|| {
                    errors.push(Error::new(
                        span,
                        format!("'{suffix_str}' is not an int suffix"),
                    ));
                    IntSuffix::Int
                }))
            },
        }
    }
}
impl IntSuffix {
    pub fn try_from_str(str: &str) -> Option<Self> {
        match str {
            "int" => Some(Self::Int),
            "int8" => Some(Self::Int8),
            "int16" => Some(Self::Int16),
            "int64" => Some(Self::Int64),
            "int128" => Some(Self::Int128),
            "intp" => Some(Self::IntPtr),
            "uint" => Some(Self::UInt),
            "uint8" => Some(Self::UInt8),
            "uint16" => Some(Self::UInt16),
            "uint64" => Some(Self::UInt64),
            "uint128" => Some(Self::UInt128),
            "uintp" => Some(Self::UIntPtr),
            _ => None,
        }
    }
}

impl Literal {
    pub fn int(span: Span, value: u128, suffix: Option<IntSuffix>) -> Self {
        Self::Int(IntLiteral::new(span, value, suffix))
    }

    pub fn int_unsuffixed(span: Span, value: u128) -> Self {
        Self::Int(IntLiteral::unsuffixed(span, value))
    }
    pub fn int8(span: Span, value: u8) -> Self {
        Self::Int(IntLiteral::int8(span, value))
    }
    pub fn int16(span: Span, value: u16) -> Self {
        Self::Int(IntLiteral::int16(span, value))
    }
    pub fn int32(span: Span, value: u32) -> Self {
        Self::Int(IntLiteral::int32(span, value))
    }
    pub fn int64(span: Span, value: u64) -> Self {
        Self::Int(IntLiteral::int64(span, value))
    }
    pub fn int128(span: Span, value: u128) -> Self {
        Self::Int(IntLiteral::int128(span, value))
    }
    pub fn intp(span: Span, value: u128) -> Self {
        Self::Int(IntLiteral::intp(span, value))
    }
    pub fn uint8(span: Span, value: u8) -> Self {
        Self::Int(IntLiteral::uint8(span, value))
    }
    pub fn uint16(span: Span, value: u16) -> Self {
        Self::Int(IntLiteral::uint16(span, value))
    }
    pub fn uint32(span: Span, value: u32) -> Self {
        Self::Int(IntLiteral::uint32(span, value))
    }
    pub fn uint64(span: Span, value: u64) -> Self {
        Self::Int(IntLiteral::uint64(span, value))
    }
    pub fn uint128(span: Span, value: u128) -> Self {
        Self::Int(IntLiteral::uint128(span, value))
    }
    pub fn uintp(span: Span, value: u128) -> Self {
        Self::Int(IntLiteral::uintp(span, value))
    }
}
