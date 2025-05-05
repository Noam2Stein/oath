use std::{cmp::Ordering, mem::replace};

use crate::*;

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "a base expression"]
pub enum BaseExpr {
    Ident(Ident),
    Literal(Literal),
    ItemKind(ItemKind),
    Out(keyword!("out")),
    Tuple(OpenParen, Vec<Expr>),
    Array(OpenBracket, Vec<Expr>),
    Block(OpenBrace, Vec<Stmt>),
    Lamba(#[span] Span, Vec<Try<VarName>>, Try<Box<Expr>>),
    UnaryOperator(#[span] Span, UnaryOperator, Try<Box<Expr>>),
}

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "an unary operator"]
pub struct UnaryExpr {
    pub base: BaseExpr,
    pub extensions: Vec<ExprExtension>,
}

#[derive(Debug, Clone, Spanned)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(#[span] Span, Box<Expr>, BinaryOperator, Try<Box<Expr>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "an unary operator"]
pub enum UnaryOperator {
    Neg(punct!("-")),
    Not(punct!("!")),
    Deref(punct!("*")),
    Ref(punct!("&"), #[option_spanned] ReferenceKind),
    Eq(punct!("==")),
    NotEq(punct!("!=")),
    More(punct!(">")),
    Less(punct!("<")),
    MoreEq(punct!(">=")),
    LessEq(punct!("<=")),
    Question(punct!("?")),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, OptionSpanned, Parse)]
pub enum ReferenceKind {
    #[fallback]
    Default,
    Mut(keyword!("mut")),
    Sole(keyword!("sole")),
    SoleMut(keyword!("smut")),
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "an expression extension"]
pub enum ExprExtension {
    Member(punct!("."), Try<Member>),
    Call(OpenParen, Vec<Expr>),
    Index(OpenBracket, Try<Box<Expr>>),
    Generics(punct!("<"), GenericArgs, punct!(">")),
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned, OptionParse)]
#[desc = "a `.` expression"]
pub enum Member {
    Unnamed(IntLiteral),
    Named(Ident),
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

impl Expr {
    fn fillin() -> Self {
        Self::Literal(Literal::Char(CharLiteral::new(
            '💪',
            Span::from_start_len(Position::new(0, 0), 1),
        )))
    }
}

impl OptionParse for Expr {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        let mut expr = Self::option_parse_no_mhs(parser)?;

        while let Some(op) = BinaryOperator::option_parse(parser) {
            match expr {
                Expr::MhsOp(_, _, expr_op, ref mut expr_rhs) if expr_op > op => {
                    let expr_rhs = match expr_rhs {
                        Try::Success(success) => success,
                        Try::Failure => {
                            parser.skip_until(|parser| <punct!(";")>::detect(parser) || <punct!(",")>::detect(parser));
                            break;
                        }
                    };

                    let lhs = Box::new(replace(&mut **expr_rhs, Self::fillin()));
                    let rhs: Try<Box<Expr>> = Self::try_parse_no_mhs(parser).map_box();

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

    fn detect(parser: &Parser) -> bool {
        UnaryExpr::detect(parser)
    }

    fn desc() -> &'static str {
        "an expression"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BinaryOperatorLevel {
    Bound,
    Or,
    And,
    Xor,
    AddSub,
    MulDivRem,
}

impl BinaryOperator {
    fn level(self) -> BinaryOperatorLevel {
        match self {
            Self::Add(_) | Self::Sub(_) => Self::AddSub,
            Self::Mul(_) | Self::Div(_) | Self::Rem(_) => BinaryOperatorLevel::MulDivRem,
            Self::Or(_) => BinaryOperatorLevel::Or,
            Self::And(_) => BinaryOperatorLevel::And,
            Self::Xor(_) => BinaryOperatorLevel::Xor,
            Self::Bound(_) => BinaryOperatorLevel::Bound,
        }
    }
}
