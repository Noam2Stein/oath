use crate::*;

pub trait FromRegexStr {
    fn from_regex_str(span: Span, str: &str, context: &Context) -> Self;
}

impl FromRegexStr for IntLiteral {
    fn from_regex_str(span: Span, str: &str, context: &Context) -> Self {
        let suffix_start = str
            .char_indices()
            .find(|(_, char)| !char.is_ascii_digit() && *char != '_')
            .map(|(char_pos, _)| char_pos);

        let int_str = &str[0..suffix_start.unwrap_or(str.len())].replace("_", "");
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let int = u128::from_str_radix(int_str, 10).unwrap_or_else(|_| {
            context.push_error(TokenError::OutOfBoundsLiteral(span));
            1
        });

        let suffix = suffix_str.map_or(None, |suffix_str| {
            Ident::new(suffix_str, span, &mut context.interner.write().unwrap()).or_else(|| {
                context.push_error(SyntaxError::Expected(span, "an ident"));

                None
            })
        });

        Self { int, suffix, span }
    }
}

impl FromRegexStr for FloatLiteral {
    fn from_regex_str(span: Span, str: &str, context: &Context) -> Self {
        let dot_position = str.char_indices().position(|(_, char)| char == '.');
        let dot_position = match dot_position {
            Some(some) => some,
            None => {
                context.push_error(SyntaxError::Expected(span, "`_._`"));
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
            context.push_error(SyntaxError::Expected(span, "`_._`"));
            return Self {
                integral: 1,
                fractional: 0,
                leading_zeros: 0,
                span,
                suffix: None,
            };
        };

        let suffix_start = str[dot_position..].char_indices().position(|(_, char)| char.is_alphabetic());

        let intergal_str = &str[0..dot_position];
        let fractional_str = &str[dot_position + 1..suffix_start.unwrap_or(str.len())];
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let integral = u128::from_str_radix(intergal_str, 10).unwrap_or_else(|_| {
            context.push_error(TokenError::OutOfBoundsLiteral(span));
            1
        });

        let leading_zeros = fractional_str.chars().position(|char| char != '0').unwrap_or(0) as u128;

        let fractional = u128::from_str_radix(fractional_str, 10).unwrap_or_else(|_| {
            context.push_error(TokenError::OutOfBoundsLiteral(span));
            1
        });

        let suffix = suffix_str.map_or(None, |suffix_str| {
            Ident::new(suffix_str, span, &mut context.interner.write().unwrap()).or_else(|| {
                context.push_error(SyntaxError::Expected(span, "an ident"));

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

impl FromRegexStr for StrLiteral {
    fn from_regex_str(span: Span, str: &str, context: &Context) -> Self {
        Self {
            str_id: context.intern(&str[1..str.len() - 1]),
            span,
        }
    }
}

impl FromRegexStr for CharLiteral {
    fn from_regex_str(span: Span, str: &str, _context: &Context) -> Self {
        Self {
            char: str.chars().skip(1).next().unwrap(),
            span,
        }
    }
}
