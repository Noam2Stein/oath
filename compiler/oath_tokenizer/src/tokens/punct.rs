use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, SpanLengthed, SpanLined, Spanned};

use crate::{with_puncts, Seal};

with_puncts!($(punct punct_len punct_variant punct_type):
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

        pub fn from_str(s: &str, span: SpanLined) -> Option<Self> {
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
);
