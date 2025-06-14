use super::*;

#[derive(Debug, OptionParse)]
#[desc = "a parameter"]
#[framed]
pub struct FramedParams<D: FrameDelimiters> {
    pub frame: Frame<D>,
    pub items: List<Param>,
    pub contract: Contract,
}

#[derive(Debug, OptionParse)]
#[desc = "a parameter"]
pub struct Param {
    pub mut_: Option<keyword!("mut")>,
    pub body: Try<ParamBody>,
    pub type_: Option<AngleUnaryExpr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, OptionParse)]
#[desc = "a parameter"]
pub enum ParamBody {
    Ident(Ident),
    Tuple(FramedParams<delims!("( )")>),
}

#[derive(Debug, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    pub colon: punct!(":"),
    pub expr: Try<Expr>,
}

impl Highlightable for Param {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {}
}
