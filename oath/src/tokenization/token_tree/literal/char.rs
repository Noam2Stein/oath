use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct CharLiteral {
    c: char,
    span: Span,
}
impl Display for CharLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.c)
    }
}
impl Spanned for CharLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl CharLiteral {
    pub fn new(span: Span, c: char) -> Self {
        Self { span, c }
    }

    pub fn from_str(span: Span, str: &str, errors: &mut ErrorsHandle) -> Self {
        let mut chars = str.chars();

        if let Some(open) = chars.next() {
            if open != '\'' {
                errors.push(Error::new(span, format!("expected ''', found '{open}'")));
            }
        } else {
            errors.push(Error::new(
                span,
                format!("expected characer literal, found empty string"),
            ));
        };

        let c = match chars.next() {
            Some('\'') => {
                errors.push(Error::new(span, "expected a character, found '''"));
                '\''
            }
            Some('\\') => match chars.next() {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('\\') => '\\',
                Some('r') => '\r',
                Some('0') => '\0',
                Some('\'') => '\'',
                Some('\"') => '\"',
                Some(other) => {
                    errors.push(Error::new(
                        span,
                        format!("unknown character escape '{other}'"),
                    ));
                    '\\'
                }
                None => {
                    errors.push(Error::new(
                        span,
                        "unexpected end of character literal, expected a character escape",
                    ));
                    '\\'
                }
            },
            Some(other) => other,
            None => {
                errors.push(Error::new(
                    span,
                    "unexpected end of character literal, expected a character",
                ));
                '\''
            }
        };

        if let Some(open) = chars.next() {
            if open != '\'' {
                errors.push(Error::new(span, format!("expected ''', found '{open}'")));
            }
        } else {
            errors.push(Error::new(
                span,
                "unexpected end of character literal, expected a '''",
            ));
        };

        Self { span, c }
    }
}

impl Literal {
    pub fn char(span: Span, c: char) -> Self {
        Self::Char(CharLiteral::new(span, c))
    }
}
