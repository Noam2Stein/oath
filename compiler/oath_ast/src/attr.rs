use super::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "an attribute"]
pub struct Attr {
    pub hash: punct!("#"),
    pub body: Try<AttrBody>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an attribute"]
pub struct InnerAttr {
    pub hash: punct!("#!"),
    pub body: Try<AttrBody>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`[ ]`"]
#[framed]
pub struct AttrBody {
    pub delims: delims!("[ ]"),
    pub ident: Try<Ident>,
    pub value: Option<AttrInput>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a meta value"]
pub enum AttrInput {
    #[framed]
    Parens(delims!("( )"), List<Expr>),
    Eq(punct!("="), Try<Expr>),
}
