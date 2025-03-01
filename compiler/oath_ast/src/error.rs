use crate::*;

#[derive(Debug, Clone, Copy, Hash)]
pub enum SyntaxError {
    Expected(Span, &'static str),
    CannotBePutOn(Span, &'static str, &'static str),
    CannotHaveTarget(Span, &'static str),
    Double(Span, &'static str),
}

impl From<SyntaxError> for Error {
    fn from(value: SyntaxError) -> Self {
        match value {
            SyntaxError::Expected(span, expected) => {
                Self::new(format!("Syntax Error: expected {expected}"), span)
            }
            SyntaxError::CannotBePutOn(span, a, b) => {
                Self::new(format!("Syntax Error: {a} cannot be put on {b}"), span)
            }
            SyntaxError::CannotHaveTarget(span, kind) => Self::new(
                format!("Syntax Error: {kind} cannot have a target item-type"),
                span,
            ),
            SyntaxError::Double(span, a) => Self::new(format!("Syntax Error: double {a}"), span),
        }
    }
}
