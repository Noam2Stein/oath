use std::{cmp::Ordering, mem::replace};

use crate::*;

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "an expr"]
pub enum Expr {
    Ident(Ident),
    ItemKind(ItemKind),
    Literal(Literal),
    Tuple(Vec<Expr>, #[span] Span),
    Array(Vec<Expr>, #[span] Span),
    Block(Block),
    Field(Box<Expr>, FieldIdent),
    Index(Box<Expr>, Box<Expr>, #[span] Span),
    Call(Box<Expr>, Vec<Expr>, #[span] Span),
    Generics(Box<Expr>, GenericArgs),
    ShsOp(ShsOp, Box<Expr>),
    MhsOp(Box<Expr>, MhsOp, Box<Expr>),
}

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "a field ident"]
pub enum FieldIdent {
    Ident(Ident),
    Int(IntLiteral),
    Unknown(Span),
}

#[derive(Debug, Clone, Spanned, ParseDesc, Parse, Detect)]
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
    #[error_fallback]
    Unknown(Span),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, ParseDesc, Parse, Detect)]
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
    #[error_fallback]
    Unknown(Span),
}

impl Expr {
    pub fn parse_no_mhs(parser: &mut Parser<impl ParserIterator>) -> Self {
        let mut expr = Self::parse_base(parser);

        loop {
            if let Some(_) = <Option<punct!(".")>>::parse(parser) {
                expr = Self::Field(
                    Box::new(replace(&mut expr, Self::fillin())),
                    Parse::parse(parser),
                )
            } else if let Some(group) = <Option<Group<Brackets>>>::parse(parser) {
                let span = expr.span() + group.span();
                expr = Self::Index(
                    Box::new(replace(&mut expr, Self::fillin())),
                    Parse::parse(&mut group.into_parser(parser.context())),
                    span,
                )
            } else if let Some(group) = <Option<Group<Parens>>>::parse(parser) {
                let span = expr.span() + group.span();
                expr = Self::Call(
                    Box::new(replace(&mut expr, Self::fillin())),
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                    span,
                )
            } else if let Some(generics) = <Option<GenericArgs>>::parse(parser) {
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

    fn parse_base(parser: &mut Parser<impl ParserIterator>) -> Self {
        if let Some(value) = Parse::parse(parser) {
            Self::Ident(value)
        } else if let Some(value) = Parse::parse(parser) {
            Self::Literal(value)
        } else if let Some(group) = <Option<Group>>::parse(parser) {
            let span = group.span();

            match group.delimiters.kind {
                DelimiterKind::Parens => Self::Tuple(
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                    span,
                ),
                DelimiterKind::Brackets => Self::Array(
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                    span,
                ),
                DelimiterKind::Braces => {
                    Self::Block(Block::parse_inner(&mut group.into_parser(parser.context())))
                }
                DelimiterKind::Angles => {
                    parser
                        .context()
                        .push_error(SyntaxError::Expected(group.span(), Self::desc()));

                    Self::Unknown
                }
            }
        } else if let Some(value) = <Option<ItemKind>>::parse(parser) {
            Self::ItemKind(value)
        } else if let Some(op) = Parse::parse(parser) {
            Self::ShsOp(op, Box::new(Self::parse_no_mhs(parser)))
        } else {
            let span = parser.peek_span();

            parser
                .context()
                .push_error(SyntaxError::Expected(span, Self::desc()));

            parser.skip_until(|parser| {
                <punct!(":")>::detect(parser) || <punct!(",")>::detect(parser)
            });

            return Self::Unknown;
        }
    }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let mut expr = Self::parse_no_mhs(parser);

        while let Some(op) = <Option<MhsOp>>::parse(parser) {
            match expr {
                Expr::MhsOp(_, expr_op, ref mut expr_rhs) if expr_op > op => {
                    **expr_rhs = Self::MhsOp(
                        Box::new(replace(&mut *expr_rhs, Self::fillin())),
                        op,
                        Box::new(Self::parse_no_mhs(parser)),
                    )
                }
                _ => {
                    expr = Self::MhsOp(
                        Box::new(replace(&mut expr, Self::fillin())),
                        op,
                        Box::new(Self::parse_no_mhs(parser)),
                    )
                }
            }
        }

        expr
    }
}

impl Detect for Expr {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Ident::detect(parser)
            || Literal::detect(parser)
            || ItemKind::detect(parser)
            || Group::<Parens>::detect(parser)
            || Group::<Braces>::detect(parser)
            || Group::<Brackets>::detect(parser)
            || ShsOp::detect(parser)
    }
}

impl Parse for FieldIdent {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        if let Some(value) = Parse::parse(parser) {
            Self::Ident(value)
        } else if let Some(value) = <Option<IntLiteral>>::parse(parser) {
            if value.suffix().is_some() {
                parser
                    .context()
                    .push_error(SyntaxError::Expected(value.span(), "no suffix"));
            }
            Self::Int(value)
        } else {
            let span = parser.peek_span();

            parser
                .context()
                .push_error(SyntaxError::Expected(span, Self::desc()));

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
                MhsOp::Unknown(_) => unreachable!(),
            }
        }

        to_lvl(self).cmp(&to_lvl(other))
    }
}
