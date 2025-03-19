use std::{cmp::Ordering, mem::replace};

use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "an expr"]
pub enum Expr {
    Path(Path),
    Type(keyword!("type")),
    Sys(keyword!("sys")),
    Trait(keyword!("trait"), ItemKind),
    Literal(Literal),
    Tuple(Span, Vec<PResult<Expr>>),
    Array(Span, Vec<PResult<Expr>>),
    Block(Block),
    ShsOp(ShsOp, Box<Expr>),
    MhsOp(Box<Expr>, MhsOp, Box<Expr>),
}

#[derive(Debug, Clone, Peek, PeekOk, Spanned)]
#[desc = "a single side op"]
pub enum ShsOp {
    Neg(punct!("-")),
    Not(punct!("!")),
    Deref(punct!("*")),
    Ref(punct!("&")),
    Eq(punct!("==")),
    NotEq(punct!("!=")),
    More(punct!(">")),
    Less(punct!("<")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),
    Question(punct!("?")),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Peek, PeekOk, Spanned)]
#[desc = "a multi side op"]
pub enum MhsOp {
    Add(punct!("+")),
    Sub(punct!("-")),
    Mul(punct!("*")),
    Div(punct!("/")),
    Rem(punct!("%")),
    BitAnd(punct!("&")),
    BitOr(punct!("|")),
    BitXor(punct!("^")),
    LogicAnd(punct!("&&")),
    LogicOr(punct!("||")),
    Eq(punct!("==")),
    NotEq(punct!("!=")),
    More(punct!(">")),
    Less(punct!("<")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),
    Bound(punct!(":")),
}

impl Expr {
    fn fillin() -> Self {
        Self::Literal(Literal::Char(CharLiteral::new('💪', Span::end_of_file())))
    }
}

impl TryParse for Expr {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        let mut expr = if let Some(group) = parser.parse::<Option<Group<Parens>>>(context) {
            Self::Tuple(
                group.span(),
                group
                    .into_parser()
                    .try_parse_trl_all::<_, punct!(",")>(context),
            )
        } else if let Some(mut value) = parser.try_parse::<Option<ItemKind>>(context)? {
            match value.keywords.pop().unwrap() {
                ItemKeyword::Type(keyword) => {
                    value.expect_empty(context, TypeKeyword::desc());
                    Self::Type(keyword)
                }
                ItemKeyword::Sys(keyword) => {
                    value.expect_empty(context, TypeKeyword::desc());
                    Self::Sys(keyword)
                }
                ItemKeyword::Trait(keyword) => Self::Trait(keyword, value),
                keyword => {
                    context.push_error(Error::new(
                        format!("`{keyword}` is not a valid expr"),
                        keyword.span(),
                    ));
                    return Err(());
                }
            }
        } else if let Some(value) = parser.parse(context) {
            Self::Sys(value)
        } else if let Some(group) = parser.parse::<Option<Group<Brackets>>>(context) {
            Self::Array(
                group.span(),
                group
                    .into_parser()
                    .try_parse_trl_all::<_, punct!(",")>(context),
            )
        } else if let Some(value) = parser.try_parse(context)? {
            Self::Path(value)
        } else if let Some(value) = parser.try_parse(context)? {
            Self::Literal(value)
        } else if let Some(op) = parser.try_parse(context)? {
            Self::ShsOp(op, parser.try_parse(context)?)
        } else {
            context.push_error(SyntaxError::Expected(parser.next_span(), "an expr"));
            return Err(());
        };

        while let Some(op) = parser.parse::<Option<MhsOp>>(context) {
            match expr {
                Expr::MhsOp(_, expr_op, ref mut expr_rhs) if expr_op > op => {
                    **expr_rhs = Self::MhsOp(
                        Box::new(replace(&mut *expr_rhs, Self::fillin())),
                        op,
                        parser.try_parse(context)?,
                    )
                }
                _ => {
                    expr = Self::MhsOp(
                        Box::new(replace(&mut expr, Self::fillin())),
                        op,
                        parser.try_parse(context)?,
                    )
                }
            }
        }

        Ok(expr)
    }
}

impl Peek for Expr {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Path>(context)
            || parser.peek::<keyword!("type")>(context)
            || parser.peek::<keyword!("sys")>(context)
            || parser.peek::<Literal>(context)
            || parser.peek::<Group>(context)
            || parser.peek::<punct!("-")>(context)
            || parser.peek::<punct!("!")>(context)
            || parser.peek::<punct!("*")>(context)
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Path(a) => a.span(),
            Self::Type(a) => a.span(),
            Self::Sys(a) => a.span(),
            Self::Trait(a, b) => a.span().connect(b.span()),
            Self::Literal(a) => a.span(),
            Self::Tuple(span, _) => *span,
            Self::Array(span, _) => *span,
            Self::Block(a) => a.span(),
            Self::ShsOp(a, b) => a.span().connect(b.span()),
            Self::MhsOp(a, b, c) => a.span().connect(b.span().connect(c.span())),
        }
    }
}

impl PartialOrd for MhsOp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MhsOp {
    fn cmp(&self, other: &Self) -> Ordering {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        enum MhsOpLvl {
            EqNotEq,
            Cmp,
            Bound,
            LogicOr,
            LogicAnd,
            BitOr,
            BitXor,
            BitAnd,
            AddSub,
            MulDivRem,
        }

        fn to_lvl(op: &MhsOp) -> MhsOpLvl {
            match op {
                MhsOp::Add(_) | MhsOp::Sub(_) => MhsOpLvl::AddSub,
                MhsOp::Mul(_) | MhsOp::Div(_) | MhsOp::Rem(_) => MhsOpLvl::MulDivRem,
                MhsOp::BitAnd(_) => MhsOpLvl::BitAnd,
                MhsOp::BitOr(_) => MhsOpLvl::BitOr,
                MhsOp::BitXor(_) => MhsOpLvl::BitXor,
                MhsOp::LogicAnd(_) => MhsOpLvl::LogicAnd,
                MhsOp::LogicOr(_) => MhsOpLvl::LogicOr,
                MhsOp::Eq(_) | MhsOp::NotEq(_) => MhsOpLvl::EqNotEq,
                MhsOp::More(_) | MhsOp::Less(_) | MhsOp::MoreEq(_) | MhsOp::LessEq(_) => {
                    MhsOpLvl::Cmp
                }
                MhsOp::Bound(_) => MhsOpLvl::Bound,
            }
        }

        to_lvl(self).cmp(&to_lvl(other))
    }
}
