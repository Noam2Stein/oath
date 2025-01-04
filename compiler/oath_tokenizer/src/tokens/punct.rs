use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, SpanLengthed, Spanned};

use crate::Seal;

macro_rules! declare_puncts {
    ($($punct:literal($punct_len:literal $punct_variant:ident $punct_type:ident), )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Punct {$(
            $punct_variant($punct_type),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $punct_type(pub SpanLengthed<$punct_len>);
        )*

        #[macro_export]
        macro_rules! punct {$(
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
                        stringify!($punct) => span.lengthed().map(|span| Self::$punct_variant($punct_type(span))),
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
                    self.0.unlined()
                }
            }
        )*
    };
}
declare_puncts!(
    ">>="(3 ShiftRAssign ShiftRAssignPunct),
    "<<="(3 ShiftLAssign ShiftLAssignPunct),
    "..."(3 DotDotDot DotDotDotPunct),
    "&&"(2 AndAnd AndAndPunct),
    "||"(2 OrOr OrOrPunct),
    "<<"(2 ShiftL ShiftLPunct),
    ">>"(2 ShiftR ShiftRPunct),
    "+="(2 PlusAssign PlusAssignPunct),
    "-="(2 MinusAssign MinusAssignPunct),
    "*="(2 StarAssign StarAssignPunct),
    "/="(2 SlashAssign SlashAssignPunct),
    "%="(2 PercentAssign PercentAssignPunct),
    "&="(2 AndAssign AndAssignPunct),
    "|="(2 OrAssign OrAssignPunct),
    "^="(2 CaretAssign CaretAssignPunct),
    "=="(2 EqEq EqEqPunct),
    "<="(2 LessEq LessEqPunct),
    ">="(2 MoreEq MoreEqPunct),
    "::"(2 ColonColon ColonColonPunct),
    ".."(2 DotDot DotDotPunct),
    "+"(1 Plus PlusPunct),
    "-"(1 Minus MinusPunct),
    "*"(1 Star StarPunct),
    "/"(1 Slash SlashPunct),
    "%"(1 Percent PercentPunct),
    "="(1 Eq EqPunct),
    "<"(1 Less LessPunct),
    ">"(1 More MorePunct),
    "&"(1 And AndPunct),
    "|"(1 Or OrPunct),
    "^"(1 Caret CaretPunct),
    "~"(1 Tilde TildePunct),
    "?"(1 Question QuestionPunct),
    "$"(1 Dollar DollarPunct),
    ","(1 Comma CommaPunct),
    ";"(1 Semi SemiPunct),
    ":"(1 Colon ColonPunct),
    "."(1 Dot DotPunct),
    "`"(1 Backtick BacktickPunct),
);
