use super::*;

pub type Expr = GenericExpr<UnOp, ExprCore, UnaryExprExt>;

#[derive(Debug, OptionSpanned, OptionParse)]
#[desc = "an expression"]
pub struct GenericExpr<P: OptionParse + Into<UnOp>, C: ParseDesc + Into<ExprCore>, E: OptionParse + Into<UnaryExprExt>> {
    #[option_spanned]
    #[parse_as(Repeated<Attr>)]
    pub attrs: Vec<Attr>,
    #[option_spanned]
    pub first_unary: Try<GenericUnaryExpr<P, C, E>>,
    #[option_spanned]
    #[parse_as(Repeated<GenericExprBinOpExt<P, C, E>>)]
    pub bin_op_exts: Vec<GenericExprBinOpExt<P, C, E>>,
}

pub type ExprBinOpExt = GenericExprBinOpExt<UnOp, ExprCore, UnaryExprExt>;

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a binary expr extension"]
pub struct GenericExprBinOpExt<P: OptionParse + Into<UnOp>, C: ParseDesc + Into<ExprCore>, E: OptionParse + Into<UnaryExprExt>> {
    pub op: BinOp,
    #[option_spanned]
    pub rhs: Try<GenericUnaryExpr<P, C, E>>,
}

// Unary Expr

pub type UnaryExpr = GenericUnaryExpr<UnOp, ExprCore, UnaryExprExt>;

#[derive(Debug, OptionSpanned, OptionParse)]
#[desc = "an expression"]
pub struct GenericUnaryExpr<P: OptionParse + Into<UnOp>, C: ParseDesc + Into<ExprCore>, E: OptionParse + Into<UnaryExprExt>> {
    #[option_spanned]
    #[parse_as(Repeated<P>)]
    pub prefixes: Vec<P>,
    #[option_spanned]
    pub core: Try<C>,
    #[option_spanned]
    #[parse_as(Repeated<E>)]
    pub exts: Vec<E>,
}

#[derive(Debug, Spanned, OptionParse)]
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

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an expression postfix"]
pub enum UnaryExprExt {
    Member(UnaryExprMemberExt),
    Call(Tuple),
    Index(Array),
    Generics(GenericArgs),
    Construct(Construct),
}

// Keyword

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an expression"]
pub enum ExprKeyword {
    Fn(keyword!("fn")),
    Out(keyword!("out")),
    Type(keyword!("type")),
}

// Member

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a member"]
pub enum Member {
    Unnamed(#[highlight(HighlightColor::Cyan)] IntLiteral),
    Named(Ident),
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`.`"]
pub struct UnaryExprMemberExt {
    pub punct: punct!("."),
    #[option_spanned]
    pub member: Try<Member>,
}

// Types

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an array"]
#[framed]
pub struct Array {
    pub frame: Frame<delims!("[ ]")>,
    #[parse_as(Trailing<Expr, punct!(",")>)]
    pub items: Vec<Expr>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a tuple"]
#[framed]
pub struct Tuple {
    pub frame: Frame<delims!("( )")>,
    #[parse_as(Trailing<Expr, punct!(",")>)]
    pub items: Vec<Expr>,
}

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`{ }`"]
#[framed]
pub struct Construct {
    pub delims: Frame<delims!("{ }")>,
    #[parse_as(Trailing<ConstructField, punct!(",")>)]
    pub items: Vec<ConstructField>,
}

#[derive(Debug, OptionParse)]
#[desc = "an identifier"]
pub struct ConstructField {
    #[highlight(HighlightColor::Cyan)]
    pub ident: Ident,
    pub set: Try<Assign>,
}

// Assign

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`=`"]
pub struct Assign {
    pub eq: punct!("="),
    #[option_spanned]
    pub value: Try<Expr>,
}

// Angle Expr

pub type AngleExpr = GenericExpr<AngleUnaryExprPrefix, ExprCore, UnaryExprExt>;
pub type AngleUnaryExpr = GenericUnaryExpr<AngleUnaryExprPrefix, ExprCore, UnaryExprExt>;
pub type AngleExprBinOpExt = GenericExprBinOpExt<AngleUnaryExprPrefix, ExprCore, UnaryExprExt>;

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an expression prefix"]
pub enum AngleUnaryExprPrefix {
    Neg(punct!("-")),
    Not(punct!("!")),
    Deref(punct!("*")),
    Ref(Ref),
    Eq(punct!("==")),
    NotEq(punct!("!=")),
    Less(punct!("<")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),
    Question(punct!("?")),
    Lifetime(Lifetime),
}

impl From<AngleUnaryExprPrefix> for UnOp {
    fn from(value: AngleUnaryExprPrefix) -> Self {
        match value {
            AngleUnaryExprPrefix::Neg(value) => Self::Neg(value.into()),
            AngleUnaryExprPrefix::Not(value) => Self::Not(value.into()),
            AngleUnaryExprPrefix::Deref(value) => Self::Deref(value.into()),
            AngleUnaryExprPrefix::Ref(value) => Self::Ref(value.into()),
            AngleUnaryExprPrefix::Eq(value) => Self::Eq(value.into()),
            AngleUnaryExprPrefix::NotEq(value) => Self::NotEq(value.into()),
            AngleUnaryExprPrefix::Less(value) => Self::Less(value.into()),
            AngleUnaryExprPrefix::MoreEq(value) => Self::MoreEq(value.into()),
            AngleUnaryExprPrefix::LessEq(value) => Self::LessEq(value.into()),
            AngleUnaryExprPrefix::Question(value) => Self::Question(value.into()),
            AngleUnaryExprPrefix::Lifetime(value) => Self::Lifetime(value.into()),
        }
    }
}

impl From<AngleExpr> for Expr {
    fn from(value: AngleExpr) -> Self {
        Self {
            attrs: value.attrs.into(),
            first_unary: value.first_unary.map(Into::into),
            bin_op_exts: value.bin_op_exts.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<AngleUnaryExpr> for UnaryExpr {
    fn from(value: AngleUnaryExpr) -> Self {
        Self {
            prefixes: value.prefixes.into_iter().map(Into::into).collect(),
            core: value.core.into(),
            exts: value.exts.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<AngleExprBinOpExt> for ExprBinOpExt {
    fn from(value: AngleExprBinOpExt) -> Self {
        Self {
            op: value.op,
            rhs: value.rhs.map(Into::into),
        }
    }
}

// Brace Expr

pub type BraceExpr = GenericExpr<UnOp, BraceExprCore, BraceUnaryExprExt>;
pub type BraceUnaryExpr = GenericUnaryExpr<UnOp, BraceExprCore, BraceUnaryExprExt>;
pub type BraceExprBinOpExt = GenericExprBinOpExt<AngleUnaryExprPrefix, ExprCore, UnaryExprExt>;

#[derive(Debug, Spanned, OptionParse)]
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

#[derive(Debug, Spanned, OptionParse)]
#[desc = "an expression postfix"]
pub enum BraceUnaryExprExt {
    Member(UnaryExprMemberExt),
    Call(Tuple),
    Index(Array),
    Generics(GenericArgs),
}

impl From<BraceExprCore> for ExprCore {
    fn from(value: BraceExprCore) -> Self {
        match value {
            BraceExprCore::Ident(value) => Self::Ident(value.into()),
            BraceExprCore::Keyword(value) => Self::Keyword(value.into()),
            BraceExprCore::Literal(value) => Self::Literal(value.into()),
            BraceExprCore::Array(value) => Self::Array(value.into()),
            BraceExprCore::Tuple(value) => Self::Tuple(value.into()),
            BraceExprCore::If(value) => Self::If(value.into()),
            BraceExprCore::Loop(value) => Self::Loop(value.into()),
            BraceExprCore::While(value) => Self::While(value.into()),
            BraceExprCore::Until(value) => Self::Until(value.into()),
            BraceExprCore::For(value) => Self::For(value.into()),
        }
    }
}

impl From<BraceUnaryExprExt> for UnaryExprExt {
    fn from(value: BraceUnaryExprExt) -> Self {
        match value {
            BraceUnaryExprExt::Member(value) => Self::Member(value.into()),
            BraceUnaryExprExt::Call(value) => Self::Call(value.into()),
            BraceUnaryExprExt::Index(value) => Self::Index(value.into()),
            BraceUnaryExprExt::Generics(value) => Self::Generics(value.into()),
        }
    }
}
