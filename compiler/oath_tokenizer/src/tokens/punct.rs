use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};
use oath_tokenizer_proc_macros::TokenDowncast;

use crate::Seal;

use super::{TokenDowncastFrom, TokenType};

pub use oath_keywords_puncts::with_puncts;

with_puncts!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, TokenDowncast)]
    pub enum Punct {$(
        $punct_variant($punct_type),
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $punct_type(pub Span);
    )*
);

pub use oath_tokenizer_proc_macros::punct;

#[allow(private_bounds)]
pub trait PunctType: TokenType + TokenDowncastFrom<Punct> {}

impl PunctType for Punct {}
impl TokenType for Punct {}
impl Seal for Punct {}

impl Spanned for Punct {
    #[inline(always)]
    fn span(&self) -> Span {
        with_puncts! {
            match self {$(
                Self::$punct_variant(keyword) => keyword.span(),
            )*}
        }
    }
}
impl Fill for Punct {
    #[inline(always)]
    fn fill(span: Span) -> Self {
        Self::Question(QuestionPunct(span))
    }
}
impl Desc for Punct {
    fn desc() -> &'static str {
        "a punct"
    }
}

with_puncts!($(
    impl PunctType for $punct_type {}
    impl TokenType for $punct_type {}
    impl Seal for $punct_type {}

    impl Spanned for $punct_type {
        #[inline(always)]
        fn span(&self) -> Span {
            self.0
        }
    }
    impl Fill for $punct_type {
        fn fill(span: Span) -> Self {
            Self(span)
        }
    }
    impl Desc for $punct_type {
        fn desc() -> &'static str {
            concat!("`", $punct, "`")
        }
    }
)*);

impl Punct {
    pub const PUNCTS: &[&str] = {
        with_puncts! {
            &[$($punct), *]
        }
    };

    pub fn from_str(s: &str, span: Span) -> Option<Self> {
        with_puncts! {
            match s {
                $($punct => Some(Self::$punct_variant($punct_type(span))),)*
                _ => None,
            }
        }
    }
}
