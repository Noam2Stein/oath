use std::fmt::{self, Display, Formatter};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdentCase {
    UpperCamelCase,
    LowerCamelCase,
}

pub trait ExpectCase {
    fn expect_case(&self, case: IdentCase, context: ContextHandle);
}

impl<T: ExpectCase> ExpectCase for Option<T> {
    fn expect_case(&self, case: IdentCase, context: ContextHandle) {
        if let Some(value) = self {
            value.expect_case(case, context);
        }
    }
}
impl<T: ExpectCase> ExpectCase for Try<T> {
    fn expect_case(&self, case: IdentCase, context: ContextHandle) {
        if let Try::Success(value) = self {
            value.expect_case(case, context);
        }
    }
}

impl Display for IdentCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpperCamelCase => write!(f, "UpperCamelCase"),
            Self::LowerCamelCase => write!(f, "lowerCamelCase"),
        }
    }
}

impl ExpectCase for Ident {
    fn expect_case(&self, case: IdentCase, context: ContextHandle) {
        let str = context.unintern(self.str_id);
        let first_char = str.chars().next().unwrap();

        let is_correct = match case {
            IdentCase::UpperCamelCase => {
                (!first_char.is_ascii_alphabetic() || first_char.is_ascii_uppercase())
                    && !str.contains("_")
            }
            IdentCase::LowerCamelCase => {
                (!first_char.is_ascii_alphabetic() || first_char.is_ascii_lowercase())
                    && !str.contains("_")
            }
        };

        if !is_correct {
            context.push_warning(Warning::new(format!("expected {case}"), self.span()));
        }
    }
}
