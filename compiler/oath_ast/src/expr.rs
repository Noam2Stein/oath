use super::*;

// EXPR TYPES

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub enum BaseExpr {
    Ident(Ident),
    Literal(Literal),
    Out(keyword!("out")),
    #[group]
    Tuple(delims!("( )"), Trailing<Expr, punct!(",")>),
    #[group]
    Array(delims!("[ ]"), Trailing<Expr, punct!(",")>),
    #[group]
    Block(delims!("{ }"), Trailing<Stmt, punct!(";")>),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct BareUnaryExpr {
    pub base: Try<BaseExpr>,
    pub postfix: Repeated<UnaryExprExtension>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct UnaryExpr {
    pub prefix: Repeated<UnaryOperator>,
    pub base: Try<BareUnaryExpr>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct BareExpr {
    pub base: BareUnaryExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct Expr {
    pub base: UnaryExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// EXPR EXTENSION TYPES

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression extension"]
pub enum UnaryExprExtension {
    Member(punct!("."), Try<Member>),
    #[group]
    Call(delims!("( )"), Trailing<Expr, punct!(",")>),
    #[group]
    Index(delims!("[ ]"), Try<Box<Expr>>),
    Generics {
        open: Discard<punct!("<")>,
        args: Trailing<BareExpr, punct!(",")>,
        close: Discard<Try<punct!(">")>>,
    },
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a binary expr extension"]
pub struct ExprBinaryPostfix {
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
