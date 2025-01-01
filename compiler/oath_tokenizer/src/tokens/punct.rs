use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, Spanned};

use crate::Seal;

macro_rules! declare_puncts {
    ($($punct:literal($punct_variant:ident $punct_type:ident), )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Punct {$(
            $punct_variant($punct_type),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $punct_type(pub Span);
        )*

        #[macro_export]
        macro_rules! Punct {$(
            ($punct) => {
                $crate::$punct_type
            };
            ($punct($span:expr)) => {
                $crate::$punct_type($span)
            };
        )*}

        #[allow(private_bounds)]
        pub trait PunctType: Seal + Send + Sync + Debug + Copy + Eq + Ord + Hash + Spanned {}

        impl PunctType for Punct {}
        impl Seal for Punct {}
        impl Spanned for Punct {
            #[inline(always)]
            fn span(&self) -> Span {
                match self {$(
                    Self::$punct_variant(keyword) => keyword.span(),
                )*}
            }
        }
        impl Punct {
            pub const PUNCTS: &[&str] = &[$(stringify!($punct)), *];

            pub fn from_str(s: &str, span: Span) -> Option<Self> {
                match s {
                    $(
                        stringify!($punct) => Some(Self::$punct_variant($punct_type(span))),
                    )*
                    _ => None,
                }
            }
        }

        $(
            impl PunctType for $punct_type {}
            impl Seal for $punct_type {}
            impl Spanned for $punct_type {
                #[inline(always)]
                fn span(&self) -> Span {
                    self.0
                }
            }
        )*
    };
}
declare_puncts!(
    ">>="(ShiftRAssign ShiftRAssignPunct),
    "<<="(ShiftLAssign ShiftLAssignPunct),
    "&&"(AndAnd AndAndPunct),
    "||"(OrOr OrOrPunct),
    "<<"(ShiftL ShiftLPunct),
    ">>"(ShiftR ShiftRPunct),
    "+="(PlusAssign PlusAssignPunct),
    "-="(MinusAssign MinusAssignPunct),
    "*="(StarAssign StarAssignPunct),
    "/="(SlashAssign SlashAssignPunct),
    "%="(PercentAssign PercentAssignPunct),
    "&="(AndAssign AndAssignPunct),
    "|="(OrAssign OrAssignPunct),
    "^="(CaretAssign CaretAssignPunct),
    "=="(EqEq EqEqPunct),
    "<="(LessEq LessEqPunct),
    ">="(MoreEq MoreEqPunct),
    "::"(ColonColon ColonColonPunct),
    "..."(DotDotDot DotDotDotPunct),
    ".."(DotDot DotDotPunct),
    "+"(Plus PlusPunct),
    "-"(Minus MinusPunct),
    "*"(Star StarPunct),
    "/"(Slash SlashPunct),
    "%"(Percent PercentPunct),
    "="(Eq EqPunct),
    "<"(Less LessPunct),
    ">"(More MorePunct),
    "&"(And AndPunct),
    "|"(Or OrPunct),
    "^"(Caret CaretPunct),
    "~"(Tilde TildePunct),
    "?"(Question QuestionPunct),
    "$"(Dollar DollarPunct),
    ","(Comma CommaPunct),
    ";"(Semi SemiPunct),
    ":"(Colon ColonPunct),
    "."(Dot DotPunct),
    "`"(Backtick BacktickPunct),
);
