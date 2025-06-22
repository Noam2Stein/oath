use super::*;

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an attribute"]
pub struct Attr {
    pub hash: punct!("#"),
    #[option_spanned]
    pub body: Try<AttrBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "an attribute"]
pub struct InnerAttr {
    pub hash: punct!("#!"),
    pub body: Try<AttrBody>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`[ ]`"]
#[framed]
pub struct AttrBody {
    pub frame: Frame<delims!("[ ]")>,
    pub ident: Try<Ident>,
    pub value: Option<AttrInput>,
}

#[derive(Debug, OptionParse)]
#[desc = "a meta value"]
pub enum AttrInput {
    Fn(Tuple),
    Set(Assign),
}
