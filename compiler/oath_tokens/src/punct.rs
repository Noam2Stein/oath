use crate::*;

#[derive(Debug, Clone, Copy, Hash, Display, Spanned)]
#[display("{kind}")]
pub struct Punct {
    #[span]
    pub span: Span,
    pub kind: PunctKind,
}

with_tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
    pub enum PunctKind {$(
        #[display($punct)]
        $punct_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, Hash, Display, Spanned)]
        #[display($punct)]
        pub struct $punct_type(#[span] pub Span);
    )*
);

verify_punct_type!(Punct);
with_tokens!(
    $(verify_punct_type!($punct_type);)*
);

pub const PUNCTS: &[&str] = with_tokens_expr! {
    &[$($punct), *]
};

pub fn is_punct(str: &str) -> bool {
    with_tokens_expr! {
        match str {
            $($punct => true,)*
            _ => false,
        }
    }
}

impl Punct {
    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        PunctKind::from_str(str).map(|kind| Self { span, kind })
    }

    pub fn as_str(self) -> &'static str {
        self.kind.as_str()
    }
}
impl PunctKind {
    pub fn from_str(str: &str) -> Option<Self> {
        with_tokens_expr! {
            match str {
                $($punct => Some(Self::$punct_variant),)*
                _ => None,
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        with_tokens_expr! {
            match self {
                $(Self::$punct_variant => $punct,)*
            }
        }
    }
}

impl<'a> TryFrom<&'a TokenTree> for Punct {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Punct(value) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

with_tokens!($(
    impl From<$punct_type> for TokenTree {
        fn from(value: $punct_type) -> Self {
            TokenTree::Punct(value.into())
        }
    }
    impl TryFrom<TokenTree> for $punct_type {
        type Error = ();
    
        fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
            if let TokenTree::Punct(value) = value {
                if value.kind == PunctKind::$punct_variant {
                    Ok($punct_type(value.span))
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
    impl<'a> TryFrom<&'a TokenTree> for $punct_type {
        type Error = ();
    
        fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
            if let TokenTree::Punct(value) = value {
                if value.kind == PunctKind::$punct_variant {
                    Ok($punct_type(value.span))
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }

    impl From<$punct_type> for Punct {
        fn from(value: $punct_type) -> Self {
            Self {
                kind: PunctKind::$punct_variant,
                span: value.span(),
            }
        }
    }
    impl TryFrom<Punct> for $punct_type {
        type Error = ();
    
        fn try_from(value: Punct) -> Result<Self, Self::Error> {
            if value.kind == PunctKind::$punct_variant {
                Ok($punct_type(value.span))
            } else {
                Err(())
            }
        }
    }
)*);

#[macro_export(local_inner_macros)]
macro_rules! verify_punct_type {
    ($type:ty) => {
        verify_token_type!($type);
        const _: () = verify_punct_type_helper::<$type>();
    };
}

#[allow(dead_code)]
pub(super) const fn verify_punct_type_helper<
    T: Debug + Copy + Eq + Ord + Hash + TryFrom<Punct> + Into<Punct> + Spanned,
>() {
}
