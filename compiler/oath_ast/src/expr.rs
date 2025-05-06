use super::*;

// EXPR TYPES

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "a base expression"]
pub enum BaseExpr {
    Ident(Ident),
    Literal(Literal),
    Out(keyword!("out")),
    UnaryOperator(UnaryOperator, #[option_spanned] Try<Box<Expr>>),
    #[group]
    Tuple(OpenParen, Vec<Expr>),
    #[group]
    Array(OpenBracket, Vec<Expr>),
    #[group]
    Block(OpenBrace, Vec<Stmt>),
}

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "an unary operator"]
pub struct UnaryExpr {
    pub base: BaseExpr,
    pub extensions: Vec<UnaryExprExtension>,
}

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "an expression"]
pub struct Expr {
    pub base: UnaryExpr,
    pub bin_ops: Vec<BinaryExprExtension>,
}

// EXPR EXTENSION TYPES

#[derive(Debug, Clone, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "an expression extension"]
pub enum UnaryExprExtension {
    Member(punct!("."), #[option_spanned] Try<Member>),
    Call(OpenParen, Vec<Expr>),
    Index(OpenBracket, Try<Box<Expr>>),
    Generics(punct!("<"), Vec<Expr>, punct!(">")),
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "a binary expr extension"]
pub struct BinaryExprExtension {
    pub op: BinaryOperator,
    pub rhs: UnaryExpr,
}

// OPERATORS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "an unary operator"]
pub enum UnaryOperator {
    Neg(punct!("-")),
    Not(punct!("!")),
    Deref(punct!("*")),
    Ref(punct!("&"), #[option_spanned] ReferenceBounds),
    Eq(punct!("==")),
    NotEq(punct!("!=")),
    More(punct!(">")),
    Less(punct!("<")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),
    Question(punct!("?")),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "a binary operator"]
pub enum BinaryOperator {
    Add(punct!("+")),
    Sub(punct!("-")),
    Mul(punct!("*")),
    Div(punct!("/")),
    Rem(punct!("%")),

    And(punct!("&")),
    Or(punct!("|")),
    Xor(punct!("^")),

    Bound(punct!(":")),
}

// ADDITIONAL TYPES

#[derive(Debug, Clone, Copy, PartialEq, Eq, OptionSpanned, Parse)]
pub enum ReferenceBounds {
    #[fallback]
    Default,
    Mut(keyword!("mut")),
    Sole(keyword!("sole")),
    SoleMut(keyword!("smut")),
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "a `.` expression"]
pub enum Member {
    Unnamed(IntLiteral),
    Named(Ident),
}
