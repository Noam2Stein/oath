use crate::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "a variable name"]
pub enum VarName {
    #[group]
    Tuple(delims!("( )"), Trailing<VarName, punct!(",")>),
    Ident(Option<keyword!("mut")>, Try<Ident>, Option<BareExpr>),
}
