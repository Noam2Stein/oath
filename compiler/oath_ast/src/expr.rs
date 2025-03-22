use std::{cmp::Ordering, mem::replace};

use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "an expr"]
pub enum Expr {
    Ident(Ident),
    ItemKind(ItemKind),
    Literal(Literal),
    Tuple(Vec<Expr>, Span),
    Array(Vec<Expr>, Span),
    Block(Block),
    Field(Box<Expr>, FieldIdent),
    Index(Box<Expr>, Box<Expr>, Span),
    Call(Box<Expr>, Vec<Expr>, Span),
    Generics(Box<Expr>, GenericArgs),
    ShsOp(ShsOp, Box<Expr>),
    MhsOp(Box<Expr>, MhsOp, Box<Expr>),
    Unknown(Span),
}

#[derive(Debug, Clone, Spanned, Desc)]
#[desc = "a field ident"]
pub enum FieldIdent {
    Ident(Ident),
    Int(IntLiteral),
    Unknown(Span),
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
    pub fn parse_no_mhs(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Self {
        let mut expr = Self::parse_base(parser, context);

        loop {
            if let Some(_) = parser.parse::<Option<punct!(".")>>(context) {
                expr = Self::Field(
                    Box::new(replace(&mut expr, Self::fillin())),
                    parser.parse(context),
                )
            } else if let Some(group) = parser.parse::<Option<Group<Brackets>>>(context) {
                let span = expr.span().connect(group.span());
                expr = Self::Index(
                    Box::new(replace(&mut expr, Self::fillin())),
                    group.into_parser().parse_all(context),
                    span,
                )
            } else if let Some(group) = parser.parse::<Option<Group<Parens>>>(context) {
                let span = expr.span().connect(group.span());
                expr = Self::Call(
                    Box::new(replace(&mut expr, Self::fillin())),
                    group.into_parser().parse_trl_all::<_, punct!(",")>(context),
                    span,
                )
            } else if let Some(generics) = parser.parse::<Option<GenericArgs>>(context) {
                expr = Self::Generics(Box::new(replace(&mut expr, Self::fillin())), generics)
            } else {
                break;
            }
        }

        expr
    }

    fn fillin() -> Self {
        Self::Literal(Literal::Char(CharLiteral::new(
            '💪',
            Span::from_start_len(Position::new(0, 0), 1),
        )))
    }

    fn parse_base(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Self {
        if let Some(value) = parser.parse(context) {
            Self::Ident(value)
        } else if let Some(value) = parser.parse(context) {
            Self::Literal(value)
        } else if let Some(group) = parser.parse::<Option<Group>>(context) {
            let span = group.span();

            match group.delimiters.kind {
                DelimiterKind::Parens => Self::Tuple(
                    group.into_parser().parse_trl_all::<_, punct!(",")>(context),
                    span,
                ),
                DelimiterKind::Brackets => Self::Array(
                    group.into_parser().parse_trl_all::<_, punct!(",")>(context),
                    span,
                ),
                DelimiterKind::Braces => {
                    Self::Block(Block::parse_inner(&mut group.into_parser(), context))
                }
                DelimiterKind::Angles => {
                    context.push_error(SyntaxError::Expected(group.span(), Self::desc()));
                    Self::Unknown(group.span())
                }
            }
        } else if let Some(value) = parser.parse::<Option<ItemKind>>(context) {
            Self::ItemKind(value)
        } else if let Some(op) = parser.parse(context) {
            Self::ShsOp(op, Box::new(Self::parse_no_mhs(parser, context)))
        } else {
            let span = parser.peek_span();

            context.push_error(SyntaxError::Expected(span, Self::desc()));

            parser.skip_until(|parser| {
                parser.peek::<punct!(":")>(context) || parser.peek::<punct!(",")>(context)
            });

            return Self::Unknown(span);
        }
    }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let mut expr = Self::parse_no_mhs(parser, context);

        while let Some(op) = parser.parse::<Option<MhsOp>>(context) {
            match expr {
                Expr::MhsOp(_, expr_op, ref mut expr_rhs) if expr_op > op => {
                    **expr_rhs = Self::MhsOp(
                        Box::new(replace(&mut *expr_rhs, Self::fillin())),
                        op,
                        Box::new(Self::parse_no_mhs(parser, context)),
                    )
                }
                _ => {
                    expr = Self::MhsOp(
                        Box::new(replace(&mut expr, Self::fillin())),
                        op,
                        Box::new(Self::parse_no_mhs(parser, context)),
                    )
                }
            }
        }

        expr
    }
}

impl Peek for Expr {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
            || parser.peek::<ItemKind>(context)
            || parser.peek::<Literal>(context)
            || parser.peek::<Group>(context)
            || parser.peek::<ShsOp>(context)
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Ident(a) => a.span(),
            Self::ItemKind(a) => a.span(),
            Self::Literal(a) => a.span(),
            Self::Tuple(_, span) => *span,
            Self::Array(_, span) => *span,
            Self::Block(a) => a.span(),
            Self::Generics(a, b) => a.span().connect(b.span()),
            Self::Field(a, b) => a.span().connect(b.span()),
            Self::Index(_, _, span) => *span,
            Self::Call(_, _, span) => *span,
            Self::ShsOp(a, b) => a.span().connect(b.span()),
            Self::MhsOp(a, b, c) => a.span().connect(b.span().connect(c.span())),
            Self::Unknown(span) => *span,
        }
    }
}

impl Parse for FieldIdent {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        if let Some(value) = parser.parse(context) {
            Self::Ident(value)
        } else if let Some(value) = parser.parse::<Option<IntLiteral>>(context) {
            if value.suffix().is_some() {
                context.push_error(SyntaxError::Expected(value.span(), "no suffix"));
            }
            Self::Int(value)
        } else {
            let span = parser.peek_span();

            context.push_error(SyntaxError::Expected(span, Self::desc()));

            parser.next();
            Self::Unknown(span)
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
