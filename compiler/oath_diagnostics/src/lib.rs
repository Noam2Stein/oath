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

#[derive(Debug, Clone, From, TryInto, Spanned)]
pub enum Error {
    Token(TokenError),
    Syntax(SyntaxError),
}

#[derive(Debug, Clone, From, TryInto, Spanned)]
pub enum Warning {
    Syntax(SyntaxWarning),
}

impl Diagnostics {
    pub fn push_error(&mut self, problem: impl Into<Error>) {
        self.errors.push(problem.into());
    }
    pub fn push_warning(&mut self, problem: impl Into<Warning>) {
        self.warnings.push(problem.into());
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }
}
