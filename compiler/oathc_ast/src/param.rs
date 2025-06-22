use super::*;

#[derive(Debug, Spanned, OptionParse, Highlightable)]
#[desc = "a parameter"]
#[framed]
pub struct FramedParams<D: FrameDelimiters> {
    pub frame: Frame<D>,
    #[highlightable]
    #[parse_as(Trailing<_, punct!(",")>)]
    pub items: Vec<Param>,
    pub contract: Contract,
}

#[derive(Debug, OptionSpanned, OptionParse, Highlightable)]
#[desc = "a parameter"]
pub struct Param {
    #[option_spanned]
    pub pub_: Option<keyword!("pub")>,
    #[option_spanned]
    pub mut_: Option<keyword!("mut")>,
    #[highlightable]
    #[option_spanned]
    pub body: Try<ParamBody>,
    #[option_spanned]
    pub type_: Option<AngleUnaryExpr>,
    #[option_spanned]
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Spanned, OptionParse, Highlightable)]
#[desc = "a parameter"]
pub enum ParamBody {
    Ident(#[highlightable] Ident),
    Tuple(#[highlightable] FramedParams<delims!("( )")>),
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    pub colon: punct!(":"),
    #[option_spanned]
    pub expr: Try<Expr>,
}
