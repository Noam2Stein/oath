use super::*;

#[derive(Debug, Clone, Hash)]
pub struct StringLiteral {
    str: Box<str>,
    span: Span,
}
impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.str)
    }
}
impl Spanned for StringLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl StringLiteral {
    pub fn new(span: Span, str: impl Into<String>) -> Self {
        Self {
            span,
            str: str.into().into_boxed_str(),
        }
    }

    pub fn from_str(span: Span, str: &str, errors: &mut ErrorsHandle) -> Self {
        let mut chars = str.chars();

        if let Some(open) = chars.next() {
            if open != '\"' {
                errors.push(Error::new(span, format!("expected '\"', found '{open}'")));
            }
        } else {
            errors.push(Error::new(
                span,
                format!("expected string literal, found empty string"),
            ));
        };

        let mut output_str = String::new();
        loop {
            match chars.next() {
                Some('\"') => break,
                Some('\\') => {
                    output_str.push('\\');
                    match chars.next() {
                        Some('n') => output_str.push('n'),
                        Some('t') => output_str.push('t'),
                        Some('\\') => output_str.push('\\'),
                        Some('r') => output_str.push('r'),
                        Some('0') => output_str.push('0'),
                        Some('\'') => output_str.push('\''),
                        Some('\"') => output_str.push('\"'),
                        Some(other) => errors.push(Error::new(
                            span,
                            format!("unknown character escape '{other}'"),
                        )),
                        None => {
                            errors.push(Error::new(
                                span,
                                "unexpected end of string literal, expected a character escape",
                            ));
                            break;
                        }
                    }
                }
                Some(other) => output_str.push(other),
                None => {
                    errors.push(Error::new(
                        span,
                        "unexpected end of string literal, expected '\"'",
                    ));
                    break;
                }
            }
        }

        Self {
            span,
            str: output_str.into_boxed_str(),
        }
    }
}

impl Literal {
    pub fn string(span: Span, str: impl Into<String>) -> Self {
        Self::String(StringLiteral::new(span, str))
    }
}
