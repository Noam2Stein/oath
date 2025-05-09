use crate::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "a variable name"]
pub enum VarName {
    #[group]
    Tuple(OpenParen, Trailing<VarName, punct!(",")>, CloseDelimiter),
    Ident(Option<keyword!("mut")>, Try<Ident>, Option<Expr>),
}
