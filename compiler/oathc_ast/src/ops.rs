use super::*;

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an expression prefix"]
pub enum UnOp {
    Not(punct!("!")),
    Neg(punct!("-")),

    Ref(Ref),
    Deref(punct!("*")),
    Lifetime(Lifetime),

    Eq(punct!("==")),
    NotEq(punct!("!=")),
    More(punct!(">")),
    Less(punct!("<")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),

    Question(punct!("?")),
    RangeExclusive(punct!("..")),
    RangeInclusive(punct!("..=")),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, OptionParse, InternedDisplay)]
#[desc = "a binary operator"]
pub enum BinOp {
    Add(punct!("+")),
    Sub(punct!("-")),
    Mul(punct!("*")),
    Div(punct!("/")),
    Rem(punct!("%")),

    And(punct!("&")),
    Or(punct!("|")),
    Xor(punct!("^")),
    Shl(punct!("<<")),
    Shr(punct!(">>")),

    Bound(punct!(":")),
    RangeExclusive(punct!("..")),
    RangeInclusive(punct!("..=")),
}

// Ref

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a reference"]
pub struct Ref {
    pub punct: punct!("&"),
    #[option_spanned]
    pub bounds: Option<RefModifier>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "reference modifier"]
pub enum RefModifier {
    Mut(keyword!("mut")),
    Sole(keyword!("sole")),
    SoleMut(keyword!("smut")),
    Lifetime(Lifetime),
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`'`"]
pub struct Lifetime {
    pub punct: punct!("'"),
    #[option_spanned]
    pub ident: Try<Ident>,
}
