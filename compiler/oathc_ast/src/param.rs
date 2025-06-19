use super::*;

#[derive(Debug, OptionParse, Highlightable)]
#[desc = "a parameter"]
#[framed]
pub struct FramedParams<D: FrameDelimiters> {
    pub frame: Frame<D>,
    #[highlightable]
    pub items: List<Param>,
    pub contract: Contract,
}

#[derive(Debug, OptionParse, Highlightable)]
#[desc = "a parameter"]
pub struct Param {
    pub mut_: Option<keyword!("mut")>,
    #[highlightable]
    pub body: Try<ParamBody>,
    pub type_: Option<AngleUnaryExpr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, OptionParse, Highlightable)]
#[desc = "a parameter"]
pub enum ParamBody {
    Ident(#[highlightable] Ident),
    Tuple(#[highlightable] FramedParams<delims!("( )")>),
}

#[derive(Debug, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    pub colon: punct!(":"),
    pub expr: Try<Expr>,
}
