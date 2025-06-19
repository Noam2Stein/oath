use super::*;

#[derive(Debug, OptionParse)]
#[desc = "`< >`"]
#[framed]
pub struct GenericArgs {
    pub frame: Frame<Angles>,
    pub args: List<Expr>,
}

#[derive(Debug, OptionParse)]
#[desc = "generic parameters"]
#[framed]
pub struct GenericParams {
    pub frame: Frame<Angles>,
    #[highlight(HighlightColor::Green)]
    pub params: List<Param>,
}
