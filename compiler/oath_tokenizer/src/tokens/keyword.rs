use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Spanned)]
pub struct Keyword {
    #[span]
    span: Span,
    pub kind: KeywordKind,
}

with_token_set!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum KeywordKind {$(
        $keyword_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Spanned)]
        pub struct $keyword_type(#[span] pub Span);
    )*
);

pub use oath_tokenizer_proc_macros::keyword;

#[allow(private_bounds)]
pub trait KeywordType: TokenType + Copy + TryFrom<Keyword> {}

impl KeywordType for Keyword {}
impl TokenType for Keyword {}
impl Seal for Keyword {}

impl TryFrom<TokenTree> for Keyword {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Keyword(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for Keyword {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Keyword(value) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

impl Highlightable for Keyword {
    fn highlight_span(&self) -> Option<Span> {
        Some(self.span)
    }
}

with_token_set!($(
    impl KeywordType for $keyword_type {}
    impl TokenType for $keyword_type {}
    impl Seal for $keyword_type {}

    impl TryFrom<TokenTree> for $keyword_type {
        type Error = ();
    
        fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
            if let TokenTree::Keyword(value) = value {
                if value.kind == KeywordKind::$keyword_variant {
                    Ok($keyword_type(value.span))
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
    impl<'a> TryFrom<&'a TokenTree> for $keyword_type {
        type Error = ();
    
        fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
            if let TokenTree::Keyword(value) = value {
                if value.kind == KeywordKind::$keyword_variant {
                    Ok($keyword_type(value.span))
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
    impl TryFrom<Keyword> for $keyword_type {
        type Error = ();
    
        fn try_from(value: Keyword) -> Result<Self, Self::Error> {
            if value.kind == KeywordKind::$keyword_variant {
                Ok($keyword_type(value.span))
            } else {
                Err(())
            }
        }
    }

    impl Display for $keyword_type {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, $keyword)
        }
    }

    impl Highlightable for $keyword_type {
        fn highlight_span(&self) -> Option<Span> {
            Some(self.0)
        }
    }    
)*);

pub const KEYWORDS: &[&str] = with_token_set_expr! {
    &[$($keyword), *]
};

pub fn is_keyword(str: &str) -> bool {
    with_token_set_expr! {
        match str {
            $($keyword => true,)*
            _ => false,
        }
    }
}

impl Keyword {
    pub fn new(kind: KeywordKind, span: Span) -> Self {
        Self { span, kind }
    }

    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        KeywordKind::from_str(str).map(|kind| Self { span, kind })
    }
}

impl KeywordKind {
    pub fn from_str(str: &str) -> Option<Self> {
        with_token_set_expr! {
            match str {
                $($keyword => Some(Self::$keyword_variant),)*
                _ => None,
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        with_token_set_expr! {
            match self {$(
                Self::$keyword_variant => $keyword,
            )*}
        }
    }
}

impl Display for KeywordKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
