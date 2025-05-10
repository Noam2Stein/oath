use oath_tokens::Ident;

use super::*;

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "a parameter"]
pub struct Param {
    #[span]
    pub name: Ident,
    pub type_: Option<BareUnaryExpr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an unnamed parameter"]
pub struct UnnamedParam {
    pub type_: BareUnaryExpr,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    pub colon: Discard<punct!(":")>,
    pub expr: Try<Expr>,
}

impl Highlight for Param {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        self.name.highlight(color, h);
    }
}
