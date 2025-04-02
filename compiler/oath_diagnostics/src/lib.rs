use derive_more::{From, TryInto};
use derive_new::new;

use oath_interner::*;
use oath_src::*;
use oath_tokens::*;

mod syntax;
mod token;
pub use syntax::*;
pub use token::*;

#[derive(Debug, Clone, new)]
pub struct Diagnostics {
    #[new(default)]
    errors: Vec<Error>,
    #[new(default)]
    warnings: Vec<Warning>,
}

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Error {
    Token(TokenError),
    Syntax(SyntaxError),
}

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Warning {
    Syntax(SyntaxWarning),
}

impl Diagnostics {
    pub fn push_error(&mut self, error: impl Into<Error>) {
        self.errors.push(error.into());
    }
    pub fn push_warning(&mut self, warning: impl Into<Warning>) {
        self.warnings.push(warning.into());
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }
}
