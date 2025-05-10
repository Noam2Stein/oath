use super::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "an attribute"]
pub struct Attr {
    pub hash: punct!("#"),
    pub meta: Try<Meta>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`[ ]`"]
#[group]
pub struct Meta {
    pub delims: delims!("[ ]"),
    pub ident: Try<Ident>,
    pub value: Option<MetaValue>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a meta value"]
pub enum MetaValue {
    #[group]
    Parens(delims!("( )"), List<Expr>),
    Eq(punct!("="), Try<Expr>),
}
