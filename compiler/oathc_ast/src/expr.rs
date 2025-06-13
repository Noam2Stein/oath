use super::*;

// Unary Expr

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub enum ExprCore {
    Ident(Ident),
    Keyword(ExprKeyword),
    Literal(Literal),
    Block(Block),
    Tuple(Tuple),
    Array(Array),
    If(If),
    Loop(Loop),
    While(While),
    Until(Until),
    For(For),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub enum ExprKeyword {
    Fn(keyword!("fn")),
    Out(keyword!("out")),
}

#[derive(Debug, OptionParse)]
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

#[derive(Debug, OptionParse)]
#[desc = "an expression postfix"]
pub enum ExprPostfix {
    Member(punct!("."), Try<Member>),
    #[framed]
    Call(delims!("( )"), List<Expr>, Leftovers),
    #[framed]
    Index(delims!("[ ]"), Try<Box<Expr>>, Leftovers),
    #[framed]
    Generics(Angles, List<AngleExpr>, Leftovers),
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a `.` expression"]
pub enum Member {
    Unnamed(#[highlight(HighlightColor::Cyan)] IntLiteral),
    Named(Ident),
}

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub struct UnaryExpr {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<ExprPrefix>,
    pub core: Try<ExprCore>,
    pub postfix: Repeated<ExprPostfix>,
}

// Binary Expr

#[derive(Debug, OptionParse)]
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

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub struct Expr {
    pub lhs: UnaryExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// ANGLE EXPR

#[derive(Debug, OptionParse)]
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

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub struct AngleUnaryExpr {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<AngleExprPrefix>,
    pub core: Try<ExprCore>,
    pub postfix: Repeated<ExprPostfix>,
}

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub struct AngleExpr {
    pub lhs: AngleUnaryExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// BRACE EXPR

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub enum BraceExprCore {
    Ident(Ident),
    Keyword(ExprKeyword),
    Literal(Literal),
    Array(Array),
    Tuple(Tuple),
    If(If),
    Loop(Loop),
    While(While),
    Until(Until),
    For(For),
}

#[derive(Debug, OptionParse)]
#[desc = "an expression postfix"]
pub enum BraceExprPostfix {
    Member(punct!("."), Try<Member>),
    #[framed]
    Call(delims!("( )"), List<Expr>, Leftovers),
    #[framed]
    Index(delims!("[ ]"), Try<Box<Expr>>, Leftovers),
    #[framed]
    Generics(Angles, List<AngleExpr>, Leftovers),
}

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub struct UnaryBraceExpr {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<ExprPrefix>,
    pub core: Try<BraceExprCore>,
    pub postfix: Repeated<BraceExprPostfix>,
}

#[derive(Debug, OptionParse)]
#[desc = "an expression"]
pub struct BraceExpr {
    pub lhs: UnaryBraceExpr,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// Anonymous Types

#[derive(Debug, OptionParse)]
#[desc = "an array"]
#[framed]
pub struct Array {
    pub delims: delims!("[ ]"),
    pub items: List<Expr>,
    pub leftovers: Leftovers,
}

#[derive(Debug, OptionParse)]
#[desc = "a tuple"]
#[framed]
pub struct Tuple {
    pub delims: delims!("( )"),
    pub items: List<Expr>,
    pub leftovers: Leftovers,
}

// If Else

#[derive(Debug, OptionParse)]
#[desc = "`if`"]
pub struct If {
    pub keyword: keyword!("if"),
    pub condition: Try<Box<BraceExpr>>,
    pub body: IfBody,
}

#[derive(Debug, Parse)]
pub enum IfBody {
    Then {
        keyword: keyword!("then"),
        expr: Try<Box<Expr>>,
        else_: Option<ThenElse>,
    },
    #[fallback]
    Block { block: Try<Block>, else_: Option<Else> },
}

#[derive(Debug, OptionParse)]
#[desc = "`else`"]
pub struct Else {
    pub keyword: keyword!("else"),
    pub body: ElseBody,
}

#[derive(Debug, Parse)]
pub enum ElseBody {
    ElseIf {
        keyword: keyword!("if"),
        condition: Try<Box<BraceExpr>>,
        body: Box<IfBody>,
    },
    #[fallback]
    Else(Try<Block>),
}

#[derive(Debug, OptionParse)]
#[desc = "an if statement"]
pub struct ThenElse {
    pub keyword: keyword!("else"),
    pub expr: Try<Box<Expr>>,
}

// Loops

#[derive(Debug, OptionParse)]
#[desc = "a loop"]
pub struct Loop {
    pub keyword: keyword!("loop"),
    pub block: Try<Block>,
}

#[derive(Debug, OptionParse)]
#[desc = "a while loop"]
pub struct While {
    pub keyword: keyword!("while"),
    pub condition: Try<Box<BraceExpr>>,
    pub block: Try<Block>,
}

#[derive(Debug, OptionParse)]
#[desc = "an until loop"]
pub struct Until {
    pub keyword: keyword!("until"),
    pub condition: Try<Box<BraceExpr>>,
    pub block: Try<Block>,
}

#[derive(Debug, OptionParse)]
#[desc = "a for loop"]
pub struct For {
    pub keyword: keyword!("for"),
    pub item: Try<Box<VarName>>,
    pub in_: Try<keyword!("in")>,
    pub iter: Try<Box<BraceExpr>>,
    pub block: Try<Block>,
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

#[derive(Debug, OptionParse)]
#[desc = "a variable name"]
pub enum VarName {
    #[framed]
    Tuple(delims!("( )"), Trailing<VarName, punct!(",")>, Leftovers),
    Ident(
        Option<keyword!("mut")>,
        #[highlight(HighlightColor::Cyan)] Try<Ident>,
        Option<AngleExpr>,
    ),
}

#[derive(Debug, OptionParse)]
#[desc = "`= ...`"]
pub struct VarInit {
    pub eq: punct!("="),
    pub init: Try<Expr>,
}

#[derive(Debug, OptionParse)]
#[desc = "`'`"]
pub struct Lifetime {
    pub punct: punct!("'"),
    pub ident: Try<Ident>,
}
