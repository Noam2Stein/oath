use super::*;

// EXPR TYPES

#[derive(Debug, Clone, OptionParse)]
#[desc = "a base expression"]
pub enum BaseExpr {
    Ident(Ident),
    Literal(Literal),
    Out(keyword!("out")),
    UnaryOperator(UnaryOperator, Try<Box<Expr>>),
    #[group]
    Tuple(OpenParen, Trailing<Expr, punct!(",")>),
    #[group]
    Array(OpenBracket, Trailing<Expr, punct!(",")>),
    #[group]
    Block(OpenBrace, Trailing<Stmt, punct!(";")>),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an unary operator"]
pub struct UnaryExpr {
    pub base: BaseExpr,
    pub extensions: Repeated<UnaryExprExtension>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct Expr {
    pub base: UnaryExpr,
    pub bin_ops: Repeated<BinaryExprExtension>,
}

// EXPR EXTENSION TYPES

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression extension"]
pub enum UnaryExprExtension {
    Member(punct!("."), Try<Member>),
    Call(OpenParen, Trailing<Expr, punct!(",")>),
    Index(OpenBracket, Try<Box<Expr>>),
    Generics(punct!("<"), Trailing<Expr, punct!(",")>, Try<punct!(">")>),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a binary expr extension"]
pub struct BinaryExprExtension {
    pub op: BinaryOperator,
    pub rhs: Try<UnaryExpr>,
}

// OPERATORS

#[derive(Debug, Clone, Copy, PartialEq, Eq, OptionParse)]
#[desc = "an unary operator"]
pub enum UnaryOperator {
    Neg(punct!("-")),
    Not(punct!("!")),
    Deref(punct!("*")),
    Ref(punct!("&"), ReferenceBounds),
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
