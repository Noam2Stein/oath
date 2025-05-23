use super::*;

// UNARY EXPR

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub enum ExprCore {
    Ident(Ident),
    Keyword(ExprKeyword),
    Literal(Literal),
    Block(Block),
    #[framed]
    Tuple(delims!("( )"), List<Expr>),
    #[framed]
    Array(delims!("[ ]"), List<Expr>),
    If {
        keyword: keyword!("if"),
        condition: Try<Box<BraceExpr>>,
        body: IfBody,
    },
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub enum ExprKeyword {
    Fn(keyword!("fn")),
    Out(keyword!("out")),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression prefix"]
pub enum ExprPrefix {
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
    Lifetime(Lifetime),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, OptionSpanned, Parse)]
pub enum ReferenceBounds {
    #[fallback]
    Default,
    Mut(keyword!("mut")),
    Sole(keyword!("sole")),
    SoleMut(keyword!("smut")),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression postfix"]
pub enum ExprPostfix {
    Member(punct!("."), Try<Member>),
    #[framed]
    Call(delims!("( )"), List<Expr>),
    #[framed]
    Index(delims!("[ ]"), Try<Box<Expr>>),
    #[framed]
    Generics(Angles, List<AngleExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "a `.` expression"]
pub enum Member {
    Unnamed(#[highlight(HighlightColor::Cyan)] IntLiteral),
    Named(Ident),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct UnaryExpr {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<ExprPrefix>,
    pub core: Try<ExprCore>,
    pub postfix: Repeated<ExprPostfix>,
}

// BINARY EXPR

#[derive(Debug, Clone, OptionParse)]
#[desc = "a binary expr extension"]
pub struct ExprBinaryPostfix {
    pub op: BinaryOperator,
    pub rhs: Try<UnaryExpr>,
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

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct Expr {
    pub lhs: UnaryExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// ANGLE EXPR

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression prefix"]
pub enum AngleExprPrefix {
    Neg(punct!("-")),
    Not(punct!("!")),
    Deref(punct!("*")),
    Ref(punct!("&"), ReferenceBounds),
    Eq(punct!("==")),
    NotEq(punct!("!=")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),
    Question(punct!("?")),
    Lifetime(Lifetime),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct AngleUnaryExpr {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<AngleExprPrefix>,
    pub core: Try<ExprCore>,
    pub postfix: Repeated<ExprPostfix>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct AngleExpr {
    pub lhs: AngleUnaryExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// BRACE EXPR

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub enum BraceExprCore {
    Ident(Ident),
    Keyword(ExprKeyword),
    Literal(Literal),
    #[framed]
    Tuple(delims!("( )"), List<Expr>),
    #[framed]
    Array(delims!("[ ]"), List<Expr>),
    If {
        keyword: keyword!("if"),
        condition: Try<Box<BraceExpr>>,
        body: IfBody,
    },
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression postfix"]
pub enum BraceExprPostfix {
    Member(punct!("."), Try<Member>),
    #[framed]
    Call(delims!("( )"), List<Expr>),
    #[framed]
    Index(delims!("[ ]"), Try<Box<Expr>>),
    #[framed]
    Generics(Angles, List<AngleExpr>),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct UnaryBraceExpr {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<ExprPrefix>,
    pub core: Try<BraceExprCore>,
    pub postfix: Repeated<BraceExprPostfix>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct BraceExpr {
    pub lhs: UnaryBraceExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// IF ELSE

#[derive(Debug, Clone, Parse)]
pub enum IfBody {
    Then {
        keyword: keyword!("then"),
        expr: Try<Box<Expr>>,
        else_: Option<ThenElse>,
    },
    #[fallback]
    Block { block: Try<Block>, else_: Option<Else> },
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`else`"]
pub struct Else {
    pub keyword: keyword!("else"),
    pub body: ElseBody,
}

#[derive(Debug, Clone, Parse)]
pub enum ElseBody {
    ElseIf {
        keyword: keyword!("if"),
        condition: Try<Box<BraceExpr>>,
        body: Box<IfBody>,
    },
    #[fallback]
    Else(Try<Block>),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an if statement"]
pub struct ThenElse {
    pub keyword: keyword!("else"),
    pub expr: Try<Box<Expr>>,
}

// LIST

pub type List<T> = Trailing<T, ListSep>;

#[derive(Debug, Clone, Copy, OptionParse)]
#[desc = "`,` / `;`"]
pub enum ListSep {
    Comma(punct!(",")),
    Semi(punct!(";")),
}

// VAR

#[derive(Debug, Clone, OptionParse)]
#[desc = "a variable name"]
pub enum VarName {
    #[framed]
    Tuple(delims!("( )"), Trailing<VarName, punct!(",")>),
    Ident(
        Option<keyword!("mut")>,
        #[highlight(HighlightColor::Cyan)] Try<Ident>,
        Option<AngleExpr>,
    ),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`= ...`"]
pub struct VarInit {
    pub eq: punct!("="),
    pub init: Try<Expr>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`'`"]
pub struct Lifetime {
    pub punct: punct!("'"),
    pub ident: Try<Ident>,
}
