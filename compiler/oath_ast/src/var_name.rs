use crate::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "a variable name"]
pub enum VarName {
    #[group]
    Tuple(delims!("( )"), Trailing<VarName, punct!(",")>),
    Ident(
        Option<keyword!("mut")>,
        #[highlight(HighlightColor::Cyan)] Try<Ident>,
        Option<BareExpr>,
    ),
}
