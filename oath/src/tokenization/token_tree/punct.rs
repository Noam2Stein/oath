use super::*;

pub use oath_proc_macros::{puncts, Punct};

puncts!(
    #[derive(Debug, Clone, Copy, Hash)]
    pub enum Punct {
        $(
            $ty_ident(Punct!($str)),
        )*
        Error(Span),
    }
    impl Display for Punct {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                $(
                    Self::$ty_ident(punct) => punct.fmt(f),
                )*
                Self::Error(_) => "".fmt(f),
            }
        }
    }
    impl Spanned for Punct {
        fn span(&self) -> Span {
            match self {
                $(
                    Self::$ty_ident(punct) => punct.span(),
                )*
                Self::Error(span) => *span,
            }
        }
    }
    impl Punct {
        pub const STRS: &'static [&'static str] = &[
            $($str), *
        ];

        pub fn try_from_str(span: Span, str: &str) -> Option<Self> {
            match str {
                $(
                    $str => Some(Self::$ty_ident(Punct!($str)(span))),
                )*
                _ => None,
            }
        }
    }

    pub mod puncts {
        use super::*;

        $(
            #[doc = $str]
            #[derive(Debug, Clone, Copy, Hash)]
            pub struct $ty_ident(pub Span);
            impl Display for $ty_ident {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    $str.fmt(f)
                }
            }
            impl Spanned for $ty_ident {
                fn span(&self) -> Span {
                    self.0
                }
            }
            impl $ty_ident {
                pub fn into_punct(self) -> Punct {
                    Punct::$ty_ident(self)
                }
            }
        )*
    }
);
