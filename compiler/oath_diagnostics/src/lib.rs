use derive_more::{From, TryInto};

use oath_interner::*;
use oath_src::*;
use oath_tokens::*;

mod name;
mod syntax;
mod token;
pub use name::*;
pub use syntax::*;
pub use token::*;

#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Diagnostic {
    Error(Error),
    Warning(Warning),
}
#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Error {
    Token(TokenError),
    Syntax(SyntaxError),
    Name(NameError),
    #[display("unfinished")]
    Unfinished(Span),
}
#[derive(Debug, Clone, From, TryInto, Spanned, InternedDisplay)]
pub enum Warning {
    Syntax(SyntaxWarning),
}
