use super::*;

#[derive(Debug, OptionParse)]
#[desc = "a struct declaration"]
pub struct Struct {
    pub keyword: keyword!("struct"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    #[highlight(HighlightColor::Cyan)]
    pub fields: Try<FramedParams<delims!("{ }")>>,
}

#[derive(Debug, OptionParse)]
#[desc = "an enum declaration"]
pub struct Enum {
    pub keyword: keyword!("enum"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    #[highlight(HighlightColor::Blue)]
    pub variants: Try<FramedParams<delims!("{ }")>>,
}
