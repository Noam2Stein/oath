use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_keywords_puncts::with_keyword_categories;
use oath_src::{Span, Spanned};
use oath_tokenizer_proc_macros::TokenDowncast;

use crate::{Seal, TokenType};

use super::TokenDowncastFrom;

pub use oath_keywords_puncts::with_keywords;

with_keywords!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, TokenDowncast)]
    pub enum Keyword {$(
        $keyword_variant($keyword_type),
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $keyword_type(pub Span);
    )*
);

pub use oath_tokenizer_proc_macros::keyword;

#[allow(private_bounds)]
pub trait KeywordType: TokenType + TokenDowncastFrom<Keyword> {}

impl KeywordType for Keyword {}
impl TokenType for Keyword {}
impl Seal for Keyword {}

impl Spanned for Keyword {
    fn span(&self) -> Span {
        with_keywords! {
            match self {$(
                Self::$keyword_variant(keyword) => keyword.span(),
            )*}
        }
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

with_keywords!($(
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
)*);

impl Keyword {
    pub fn is_keyword(s: &str) -> bool {
        with_keywords! {
            match s {
                $(stringify!($keyword) => true,)*
                _ => false,
            }
        }
    }

    pub fn from_str(s: &str, span: Span) -> Option<Self> {
        with_keywords! {
            match s {
                $(stringify!($keyword) => Some(Self::$keyword_variant($keyword_type(span))),)*
                _ => None,
            }
        }
    }
}

with_keyword_categories!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum KeywordCategory {$(
        $category,
    )*}
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeywordInfo {
    pub str: &'static str,
    pub category: KeywordCategory,
}

impl Keyword {
    pub const KEYWORDS: &[KeywordInfo] = {
        with_keywords! { &[$(
            KeywordInfo {
                str: stringify!($keyword),
                category: KeywordCategory::$keyword_category,
            }
        ), *]}
    };
}
