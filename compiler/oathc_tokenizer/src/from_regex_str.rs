use super::*;

pub trait FromRegexStr {
    fn from_regex_str(span: Span, str: &str, interner: &Interner, diagnostics: &Diagnostics) -> Self;
}

impl FromRegexStr for LiteralSuffix {
    fn from_regex_str(span: Span, str: &str, interner: &Interner, diagnostics: &Diagnostics) -> Self {
        match Ident::new(str, span, &interner) {
            Some(ident) => Self {
                ident: Try::Success(ident),
            },
            None => Self {
                ident: Try::Failure(Some(diagnostics.push_error(Error::Expected(span, "an ident")))),
            },
        }
    }
}

impl FromRegexStr for IntLiteral {
    fn from_regex_str(span: Span, str: &str, interner: &Interner, diagnostics: &Diagnostics) -> Self {
        let suffix_start = str
            .char_indices()
            .find(|(_, char)| !char.is_ascii_digit() && *char != '_')
            .map(|(char_pos, _)| char_pos);

        let value_str = &str[0..suffix_start.unwrap_or(str.len())].replace("_", "");
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let value = u128::from_str_radix(value_str, 10).map_or_else(
            |_| Try::Failure(Some(diagnostics.push_error(Error::OutOfBoundsLiteral(span)))),
            |value| Try::Success(value),
        );

        let suffix = suffix_str.map(|suffix_str| LiteralSuffix::from_regex_str(span, suffix_str, interner, diagnostics));

        Self { value, suffix, span }
    }
}

impl FromRegexStr for FloatLiteral {
    fn from_regex_str(span: Span, str: &str, interner: &Interner, diagnostics: &Diagnostics) -> Self {
        let dot_position = str.char_indices().position(|(_, char)| char == '.');
        let dot_position = match dot_position {
            Some(dot_position) if str.len() > dot_position + 1 => dot_position,
            _ => {
                let integral = Try::Failure(None);
                let fractional = Try::Failure(Some(diagnostics.push_error(Error::Expected(span, "`_._`"))));

                return Self {
                    value_integral: integral,
                    value_fraction: fractional,
                    value_leading_zeros: 0,
                    span,
                    suffix: None,
                };
            }
        };

        let suffix_start = str[dot_position..].char_indices().position(|(_, char)| char.is_alphabetic());

        let value_intergal_str = &str[0..dot_position];
        let value_fraction_str = &str[dot_position + 1..suffix_start.unwrap_or(str.len())];
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let value_integral = u128::from_str_radix(value_intergal_str, 10).map_or_else(
            |_| Try::Failure(Some(diagnostics.push_error(Error::OutOfBoundsLiteral(span)))),
            |value| Try::Success(value),
        );

        let value_leading_zeros = value_fraction_str.chars().position(|char| char != '0').unwrap_or(0) as u128;

        let value_fraction = u128::from_str_radix(value_fraction_str, 10).map_or_else(
            |_| Try::Failure(Some(diagnostics.push_error(Error::OutOfBoundsLiteral(span)))),
            |value| Try::Success(value),
        );

        let suffix = suffix_str.map(|suffix_str| LiteralSuffix::from_regex_str(span, suffix_str, interner, diagnostics));

        Self {
            value_integral,
            value_leading_zeros,
            value_fraction,
            span,
            suffix,
        }
    }
}

impl FromRegexStr for StrLiteral {
    fn from_regex_str(span: Span, str: &str, interner: &Interner, _diagnostics: &Diagnostics) -> Self {
        Self {
            str_id: interner.intern(&str[1..str.len() - 1]),
            span,
        }
    }
}

impl FromRegexStr for CharLiteral {
    fn from_regex_str(span: Span, str: &str, _interner: &Interner, _diagnostics: &Diagnostics) -> Self {
        Self {
            char: str.chars().skip(1).next().unwrap(),
            span,
        }
    }
}
