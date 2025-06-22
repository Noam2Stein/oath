use super::*;

// If Else

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`if`"]
pub struct If {
    pub keyword: keyword!("if"),
    #[option_spanned]
    pub condition: Try<Box<BraceExpr>>,
    #[option_spanned]
    pub body: IfBody,
}

#[derive(Debug, OptionSpanned, Parse)]
pub enum IfBody {
    Then(#[option_spanned] IfThen),
    #[fallback]
    Block(#[option_spanned] IfBlock),
}

#[derive(Debug, OptionSpanned, Parse)]
pub struct IfBlock {
    #[option_spanned]
    pub block: Try<Block>,
    #[option_spanned]
    pub else_: Option<ThenElse>,
}

#[derive(Debug, OptionParse)]
#[desc = "`else`"]
pub struct Else {
    pub keyword: keyword!("else"),
    pub body: ElseBody,
}

#[derive(Debug, Parse)]
pub enum ElseBody {
    ElseIf(Box<If>),
    #[fallback]
    Else(Try<Block>),
}

// Then

#[derive(Debug, OptionSpanned, OptionParse)]
#[desc = "then"]
pub struct IfThen {
    pub keyword: keyword!("then"),
    #[option_spanned]
    pub expr: Try<Box<Expr>>,
    #[option_spanned]
    pub else_: Option<ThenElse>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`else`"]
pub struct ThenElse {
    pub keyword: keyword!("else"),
    #[option_spanned]
    pub expr: Try<Box<Expr>>,
}

// Loops

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a loop"]
pub struct Loop {
    pub keyword: keyword!("loop"),
    #[option_spanned]
    pub block: Try<Block>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a while loop"]
pub struct While {
    pub keyword: keyword!("while"),
    #[option_spanned]
    pub condition: Try<Box<BraceExpr>>,
    #[option_spanned]
    pub block: Try<Block>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an until loop"]
pub struct Until {
    pub keyword: keyword!("until"),
    #[option_spanned]
    pub condition: Try<Box<BraceExpr>>,
    #[option_spanned]
    pub block: Try<Block>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a for loop"]
pub struct For {
    pub keyword: keyword!("for"),
    #[highlight(HighlightColor::Cyan)]
    #[option_spanned]
    pub item: Try<Box<Param>>,
    #[option_spanned]
    pub in_: Try<keyword!("in")>,
    #[option_spanned]
    pub iter: Try<Box<BraceExpr>>,
    #[option_spanned]
    pub block: Try<Block>,
}
