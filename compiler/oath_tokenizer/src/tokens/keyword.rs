use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};
use oath_tokenizer_macros::TokenDowncast;

use crate::{with_keywords, Seal, TokenType};

use super::TokenDowncastFrom;

macro_rules! use_keywords {
    ($($keyword:ident($keyword_len:literal $keyword_variant:ident $keyword_type:ident), )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, TokenDowncast)]
        pub enum Keyword {$(
            $keyword_variant($keyword_type),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $keyword_type(pub Span);
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
        pub trait KeywordType: TokenType + TokenDowncastFrom<Keyword> {}

        impl KeywordType for Keyword {}
        impl TokenType for Keyword {}
        impl Seal for Keyword {}
        impl Spanned for Keyword {
            #[inline(always)]
            fn span(&self) -> Span {
                match self {$(
                    Self::$keyword_variant(keyword) => keyword.span(),
                )*}
            }
        }
        impl Fill for Keyword {
            fn fill(span: Span) -> Self {
                Self::Mod(ModKeyword(span))
            }
        }
        impl Desc for Keyword {
            fn desc() -> &'static str {
                "a keyword"
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

            pub fn from_str(s: &str, span: Span) -> Option<Self> {
                match s {
                    $(
                        stringify!($keyword) => Some(Self::$keyword_variant($keyword_type(span))),
                    )*
                    _ => None,
                }
            }
        }

        $(
            impl KeywordType for $keyword_type {}
            impl TokenType for $keyword_type {}
            impl Seal for $keyword_type {}
            impl Spanned for $keyword_type {
                #[inline(always)]
                fn span(&self) -> Span {
                    self.0
                }
            }
            impl Fill for $keyword_type {
                fn fill(span: Span) -> Self {
                    Self(span)
                }
            }
            impl Desc for $keyword_type {
                fn desc() -> &'static str {
                    concat!("`", stringify!($keyword), "`")
                }
            }
        )*
    };
}
with_keywords!(use_keywords);
