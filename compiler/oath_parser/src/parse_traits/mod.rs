mod option_parse;
mod parse;
pub use option_parse::*;
pub use parse::*;

mod token_impl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseExit {
    Complete,
    Cut,
}
