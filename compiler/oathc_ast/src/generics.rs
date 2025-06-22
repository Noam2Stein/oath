use super::*;

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`< >`"]
#[framed]
pub struct GenericArgs {
    pub frame: Frame<Angles>,
    #[parse_as(Trailing<_, punct!(",")>)]
    pub args: Vec<Expr>,
}

#[derive(Debug, OptionParse)]
#[desc = "generic parameters"]
#[framed]
pub struct GenericParams {
    pub frame: Frame<Angles>,
    #[highlight(HighlightColor::Green)]
    #[parse_as(Trailing<_, punct!(",")>)]
    pub params: Vec<Param>,
}
