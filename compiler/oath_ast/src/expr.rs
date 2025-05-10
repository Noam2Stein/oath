use super::*;

// EXPR TYPES

#[derive(Debug, Clone, OptionParse)]
#[desc = "a base expression"]
pub enum BaseExpr {
    Ident(Ident),
    Literal(Literal),
    Out(keyword!("out")),
    Block(Block),
    Tuple(Tuple),
    Array(Array),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a strict base expression"]
pub enum StrictBaseExpr {
    Ident(Ident),
    Literal(Literal),
    Out(keyword!("out")),
    Tuple(Tuple),
    Array(Array),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a bare unary expression"]
pub struct BareUnaryExpr<B: ParseDesc + Debug + Clone = BaseExpr> {
    pub attrs: Repeated<Attr>,
    pub base: Try<B>,
    pub postfix: Repeated<ExprPostfix>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an unary expression"]
pub struct UnaryExpr<B: ParseDesc + Debug + Clone = BaseExpr> {
    pub attrs: Repeated<Attr>,
    pub prefix: Repeated<ExprPrefix>,
    pub base: Try<B>,
    pub postfix: Repeated<ExprPostfix>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a bare expression"]
pub struct BareExpr<B: ParseDesc + Debug + Clone = BaseExpr> {
    pub base: BareUnaryExpr<B>,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression"]
pub struct Expr<B: ParseDesc + Debug + Clone = BaseExpr> {
    pub base: UnaryExpr<B>,
    pub bin_ops: Repeated<ExprBinaryPostfix>,
}

// UNARY TYPES

#[derive(Debug, Clone, Copy, PartialEq, Eq, OptionParse)]
#[desc = "an unary operator"]
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
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an expression extension"]
pub enum ExprPostfix {
    Member(punct!("."), Try<Member>),
    Call(Tuple),
    #[group]
    Index(delims!("[ ]"), Try<Box<Expr>>),
    Generics {
        open: Discard<punct!("<")>,
        args: Trailing<BareExpr, punct!(",")>,
        close: Discard<Try<punct!(">")>>,
    },
}

// BINARY

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

// ADDITIONAL TYPES

#[derive(Debug, Clone, OptionParse)]
#[desc = "`{ }`"]
#[group]
pub struct Block {
    pub delims: delims!("{ }"),
    pub values: List<Stmt>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`( )`"]
#[group]
pub struct Tuple {
    pub delims: delims!("( )"),
    pub values: List<Expr>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`[ ]`"]
#[group]
pub struct Array {
    pub delims: delims!("[ ]"),
    pub values: List<Expr>,
}

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
    Unnamed(#[highlight(HighlightColor::Cyan)] IntLiteral),
    Named(Ident),
}

pub type List<T> = Trailing<T, ListSep>;

#[derive(Debug, Clone, Copy, OptionParse)]
#[desc = "`,` / `;`"]
pub enum ListSep {
    Comma(punct!(",")),
    Semi(punct!(";")),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a variable name"]
pub enum VarName {
    #[group]
    Tuple(delims!("( )"), Trailing<VarName, punct!(",")>),
    Ident(
        Option<keyword!("mut")>,
        #[highlight(HighlightColor::Cyan)] Try<Ident>,
        Option<BareExpr>,
    ),
}
