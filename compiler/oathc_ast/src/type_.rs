use super::*;

#[derive(Debug, OptionParse)]
#[desc = "a type declaration"]
pub struct Type {
    pub keyword: keyword!("type"),
    pub ident: Try<Ident>,
    pub generics: Option<FramedParams<Angles>>,
    pub contract: Contract,
    pub semi: Try<punct!(";")>,
}

#[derive(Debug, OptionParse)]
#[desc = "a struct declaration"]
pub struct Struct {
    pub keyword: keyword!("struct"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<FramedParams<Angles>>,
    pub contract: Contract,
    pub fields: Try<FramedParams<delims!("{ }")>>,
}

#[derive(Debug, OptionParse)]
#[desc = "an enum declaration"]
pub struct Enum {
    pub keyword: keyword!("enum"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<FramedParams<Angles>>,
    pub contract: Contract,
    pub variants: Try<FramedParams<delims!("{ }")>>,
}
