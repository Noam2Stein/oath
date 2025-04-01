use crate::*;

#[derive(Debug, Clone, Copy, Hash, Display, new, Spanned)]
#[display("{kind}")]
pub struct Keyword {
    #[span]
    pub span: Span,
    pub kind: KeywordKind,
}

with_tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
    pub enum KeywordKind {$(
        #[display($keyword)]
        $keyword_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, Hash, Display, Spanned)]
        #[display($keyword)]
        pub struct $keyword_type(#[span] pub Span);
    )*
);

verify_keyword_type!(Keyword);
with_tokens!(
    $(verify_keyword_type!($keyword_type);)*
);

pub const KEYWORDS: &[&str] = with_tokens_expr! {
    &[$($keyword), *]
};

pub fn is_keyword(str: &str) -> bool {
    with_tokens_expr! {
        match str {
            $($keyword => true,)*
            _ => false,
        }
    }
}

impl Keyword {
    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        KeywordKind::from_str(str).map(|kind| Self { span, kind })
    }

    pub fn as_str(self) -> &'static str {
        self.kind.as_str()
    }
}
impl KeywordKind {
    pub fn from_str(str: &str) -> Option<Self> {
        with_tokens_expr! {
            match str {
                $($keyword => Some(Self::$keyword_variant),)*
                _ => None,
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$keyword_variant => $keyword,
            )*}
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

with_tokens!($(
    impl From<$keyword_type> for TokenTree {
        fn from(value: $keyword_type) -> Self {
            TokenTree::Keyword(value.into())
        }
    }
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

    impl From<$keyword_type> for Keyword {
        fn from(value: $keyword_type) -> Self {
            Self {
                kind: KeywordKind::$keyword_variant,
                span: value.span(),
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
)*);

#[macro_export(local_inner_macros)]
macro_rules! verify_keyword_type {
    ($type:ty) => {
        verify_token_type!($type);
        const _: () = verify_keyword_type_helper::<$type>();
    };
}

#[allow(dead_code)]
pub(super) const fn verify_keyword_type_helper<
    T: Debug + Copy + Eq + Ord + Hash + TryFrom<Keyword> + Into<Keyword> + Spanned,
>() {
}
