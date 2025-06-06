use super::*;

#[derive(Debug, OptionParse)]
#[desc = "an attribute"]
pub struct Attr {
    pub hash: punct!("#"),
    pub body: Try<AttrBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "an attribute"]
pub struct InnerAttr {
    pub hash: punct!("#!"),
    pub body: Try<AttrBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "`[ ]`"]
#[framed]
pub struct AttrBody {
    pub delims: delims!("[ ]"),
    pub ident: Try<Ident>,
    pub value: Option<AttrInput>,
    pub leftovers: Leftovers,
}

#[derive(Debug, OptionParse)]
#[desc = "a meta value"]
pub enum AttrInput {
    #[framed]
    Parens(delims!("( )"), List<Expr>, Leftovers),
    Eq(punct!("="), Try<Expr>),
}
