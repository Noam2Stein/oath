use super::*;

pub use oath_proc_macros::{keywords, Keyword};

keywords!(
    #[derive(Debug, Clone, Copy, Hash)]
    pub enum Keyword {
        $(
            $ty_ident(Keyword!($str)),
        )*
        Error(Span),
    }
    impl Display for Keyword {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                $(
                    Self::$ty_ident(keyword) => keyword.fmt(f),
                )*
                Self::Error(_) => "".fmt(f),
            }
        }
    }
    impl Spanned for Keyword {
        fn span(&self) -> Span {
            match self {
                $(
                    Self::$ty_ident(keyword) => keyword.span(),
                )*
                Self::Error(span) => *span,
            }
        }
    }
    impl Keyword {
        pub const STRS: &'static [&'static str] = &[
            $($str), *
        ];

        pub fn try_from_str(span: Span, str: &str) -> Option<Self> {
            match str {
                $(
                    $str => Some(Self::$ty_ident(Keyword!($str)(span))),
                )*
                _ => None,
            }
        }
    }

    pub mod keywords {
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
                pub fn into_keyword(self) -> Keyword {
                    Keyword::$ty_ident(self)
                }
            }
        )*
    }
);
