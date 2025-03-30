use std::{cmp::Ordering, mem::replace};

use crate::*;

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "an expr"]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    ItemKind(ItemKind),
    Out(keyword!("out")),
    Tuple(#[span] Span, Vec<Try<Expr>>),
    Array(#[span] Span, Vec<Try<Expr>>),
    Block(Block),
    Field(Box<Expr>, FieldIdent),
    Index(#[span] Span, Box<Expr>, Try<Box<Expr>>),
    Call(#[span] Span, Box<Expr>, Vec<Try<Expr>>),
    Generics(Box<Expr>, GenericArgs),
    Lamba(#[span] Span, Vec<Try<VarName>>, Try<Box<Expr>>),
    ShsOp(#[span] Span, ShsOp, Try<Box<Expr>>),
    MhsOp(#[span] Span, Box<Expr>, MhsOp, Try<Box<Expr>>),
}

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "a field ident"]
pub enum FieldIdent {
    Ident(Ident),
    Int(IntLiteral),
    Unknown(Span),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, OptionParse)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, OptionParse)]
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
    pub fn try_parse_no_mhs(parser: &mut Parser<impl ParserIterator>) -> Try<Self> {
        if let Some(output) = Self::option_parse_no_mhs(parser) {
            Try::Success(output)
        } else {
            parser.context().push_error(Error::new(
                format!("Syntax Error: expected {}", Self::desc()),
                parser.peek_span(),
            ));

            Try::Failure
        }
    }

    pub fn option_parse_no_mhs(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let mut expr = Self::option_parse_base(parser)?;

        loop {
            if let Some(_) = <punct!(".")>::option_parse(parser) {
                let base = Box::new(replace(&mut expr, Self::fillin()));
                let field = Parse::parse(parser);

                expr = Self::Field(base, field);

                continue;
            }

            if let Some(group) = Group::<Brackets>::option_parse(parser) {
                let span = expr.span() + group.span();
                let base = Box::new(replace(&mut expr, Self::fillin()));
                let index = Expr::try_parse(&mut group.into_parser(parser.context())).map_box();

                expr = Self::Index(span, base, index);

                continue;
            }

            if let Some(group) = Group::<Parens>::option_parse(parser) {
                expr = Self::Call(
                    expr.span() + group.span(),
                    Box::new(replace(&mut expr, Self::fillin())),
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                );

                continue;
            }

            if let Some(generics) = GenericArgs::option_parse(parser) {
                expr = Self::Generics(Box::new(replace(&mut expr, Self::fillin())), generics);

                continue;
            }

            break;
        }

        Some(expr)
    }

    fn fillin() -> Self {
        Self::Literal(Literal::Char(CharLiteral::new(
            '💪',
            Span::from_start_len(Position::new(0, 0), 1),
        )))
    }

    fn option_parse_base(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        if let Some(value) = Parse::parse(parser) {
            return Some(Self::Ident(value));
        }

        if let Some(value) = Parse::parse(parser) {
            return Some(Self::Literal(value));
        }

        if let Some(group) = Group::<Parens>::option_parse(parser) {
            return Some(Self::Tuple(
                group.span(),
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>(),
            ));
        }

        if let Some(group) = Group::<Brackets>::option_parse(parser) {
            return Some(Self::Array(
                group.span(),
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>(),
            ));
        }

        if let Some(open_punct) = <punct!("|")>::option_parse(parser) {
            let param_names = parser.parse_trl::<Try<VarName>, punct!(",")>();

            if <punct!("|")>::try_parse(parser).is_failure() {
                return Some(Self::Lamba(
                    open_punct.span() + param_names.last().map_or(None, |name| name.option_span()),
                    param_names,
                    Try::Failure,
                ));
            }

            let expr = Expr::try_parse(parser).map_box();

            let span = open_punct.span() + expr.option_span();

            return Some(Self::Lamba(span, param_names, expr));
        }

        if let Some(input) = <punct!("||")>::option_parse(parser) {
            let expr = Expr::try_parse(parser).map_box();

            let span = input.span() + expr.option_span();

            return Some(Self::Lamba(span, Vec::new(), expr));
        }

        if let Some(value) = Parse::parse(parser) {
            return Some(Self::Block(value));
        }

        if let Some(value) = Parse::parse(parser) {
            return Some(Self::ItemKind(value));
        }

        if let Some(value) = Parse::parse(parser) {
            return Some(Self::Out(value));
        }

        if let Some(shs_op) = ShsOp::option_parse(parser) {
            let expr = Self::try_parse_no_mhs(parser).map_box();

            let span = shs_op.span() + expr.option_span().unwrap_or(parser.peek_span());

            return Some(Self::ShsOp(span, shs_op, expr));
        }

        None
    }
}

impl OptionParse for Expr {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let mut expr = Self::option_parse_no_mhs(parser)?;

        while let Some(op) = MhsOp::option_parse(parser) {
            match expr {
                Expr::MhsOp(_, _, expr_op, ref mut expr_rhs) if expr_op > op => {
                    let expr_rhs = match expr_rhs {
                        Try::Success(success) => success,
                        Try::Failure => {
                            parser.skip_until(|parser| {
                                <punct!(";")>::detect(parser) || <punct!(",")>::detect(parser)
                            });
                            break;
                        }
                    };

                    let lhs = Box::new(replace(&mut **expr_rhs, Self::fillin()));
                    let rhs = Self::try_parse_no_mhs(parser).map_box();

                    let span = lhs.span() + rhs.option_span().unwrap_or(op.span());

                    **expr_rhs = Self::MhsOp(span, lhs, op, rhs);
                }
                _ => {
                    let lhs = Box::new(replace(&mut expr, Self::fillin()));
                    let rhs = Self::try_parse_no_mhs(parser).map_box();

                    let span = lhs.span() + rhs.option_span().unwrap_or(op.span());

                    expr = Self::MhsOp(span, lhs, op, rhs);
                }
            }
        }

        Some(expr)
    }
}

impl Detect for Expr {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Ident::detect(parser)
            || Literal::detect(parser)
            || Group::<Parens>::detect(parser)
            || Group::<Braces>::detect(parser)
            || Group::<Brackets>::detect(parser)
            || ItemKind::detect(parser)
            || <keyword!("out")>::detect(parser)
            || <punct!("|")>::detect(parser)
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
            }
        }

        to_lvl(self).cmp(&to_lvl(other))
    }
}
