use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Punct {
    span: Span,
    pub kind: PunctKind,
}

with_token_set!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PunctKind {$(
        $punct_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $punct_type(pub Span);
    )*
);

pub use oath_tokenizer_proc_macros::punct;

#[allow(private_bounds)]
pub trait PunctType: TokenType + Copy + TryFrom<Punct> {}

impl PunctType for Punct {}
impl TokenType for Punct {}
impl Seal for Punct {}

impl TryFrom<TokenTree> for Punct {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Punct(value) = value {
            Ok(value)
        } else {
            Err(())
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

impl Spanned for Punct {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

with_token_set!($(
    impl PunctType for $punct_type {}
    impl TokenType for $punct_type {}
    impl Seal for $punct_type {}

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

    impl Spanned for $punct_type {
        #[inline(always)]
        fn span(&self) -> Span {
            self.0
        }
    }
)*);

pub const PUNCTS: &[&str] = with_token_set_expr! {
    &[$($punct), *]
};

impl Punct {
    pub fn new(kind: PunctKind, span: Span) -> Self {
        Self { span, kind }
    }

    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        PunctKind::from_str(str).map(|kind| Self { span, kind })
    }
}

impl PunctKind {
    pub fn from_str(str: &str) -> Option<Self> {
        with_token_set_expr! {
            match str {
                $($punct => Some(Self::$punct_variant),)*
                _ => None,
            }
        }
    }
}
