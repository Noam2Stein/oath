use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, SpanLengthed, SpanLined, Spanned};

use crate::{with_keywords, Seal};

macro_rules! use_keywords {
    ($($keyword:ident($keyword_len:literal $keyword_variant:ident $keyword_type:ident), )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Keyword {$(
            $keyword_variant($keyword_type),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $keyword_type(pub SpanLengthed<$keyword_len>);
        )*

        #[macro_export]
        macro_rules! keyword {$(
            ($keyword) => {
                $crate::$keyword_type
            };
            ($keyword($span:expr)) => {
                $crate::$keyword_type($span)
            };
        )*}

        #[allow(private_bounds)]
        pub trait KeywordType: Seal + Send + Sync + Debug + Copy + Eq + Ord + Hash + Spanned {}

        impl KeywordType for Keyword {}
        impl Seal for Keyword {}
        impl Spanned for Keyword {
            #[inline(always)]
            fn span(&self) -> Span {
                match self {$(
                    Self::$keyword_variant(keyword) => keyword.span(),
                )*}
            }
        }
        impl Keyword {
            pub const KEYWORDS: &[&str] = &[$(stringify!($keyword)), *];

            pub fn is_keyword(s: &str) -> bool {
                match s {
                    $(
                        stringify!($keyword) => true,
                    )*
                    _ => false,
                }
            }

            pub fn from_str(s: &str, span: SpanLined) -> Option<Self> {
                match s {
                    $(
                        stringify!($keyword) => span.lengthed().map(|span| Self::$keyword_variant($keyword_type(span))),
                    )*
                    _ => None,
                }
            }
        }

        $(
            impl KeywordType for $keyword_type {}
            impl Seal for $keyword_type {}
            impl Spanned for $keyword_type {
                #[inline(always)]
                fn span(&self) -> Span {
                    self.0.unlined()
                }
            }
        )*
    };
}
with_keywords!(use_keywords);
