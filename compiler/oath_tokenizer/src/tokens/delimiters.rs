use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::TokenDowncastFrom;

pub use oath_keywords_puncts::with_delimiters;

with_delimiters!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Delimiters {$(
        $delim_type($delim_type),
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $delim_type {
            open_span: Span,
            close_span: Span,
        }
    )*
);

#[allow(private_bounds)]
pub trait DelimitersType:
    Seal
    + Send
    + Sync
    + Debug
    + Copy
    + Eq
    + Ord
    + Hash
    + Spanned
    + Fill
    + Desc
    + TokenDowncastFrom<Delimiters>
{
    fn open_span(self) -> Span;
    fn close_span(self) -> Span;
}

impl DelimitersType for Delimiters {
    #[inline(always)]
    fn open_span(self) -> Span {
        with_delimiters! {
            match self {$(
                Self::$delim_type(delim) => delim.open_span(),
            )*}
        }
    }
    #[inline(always)]
    fn close_span(self) -> Span {
        with_delimiters! {
            match self {$(
                Self::$delim_type(delim) => delim.close_span(),
            )*}
        }
    }
}
impl Seal for Delimiters {}

impl Spanned for Delimiters {
    #[inline(always)]
    fn span(&self) -> Span {
        self.open_span().connect(self.close_span())
    }
}
impl Fill for Delimiters {
    fn fill(span: Span) -> Self {
        Self::Parens(Parens::new(span, span))
    }
}
impl Desc for Delimiters {
    fn desc() -> &'static str {
        "delimiters"
    }
}

with_delimiters!(
    $(
        impl DelimitersType for $delim_type {
            #[inline(always)]
            fn open_span(self) -> Span {
                self.open_span
            }
            #[inline(always)]
            fn close_span(self) -> Span {
                self.close_span
            }
        }
        impl Seal for $delim_type {}

        impl Spanned for $delim_type {
            #[inline(always)]
            fn span(&self) -> Span {
                self.open_span().connect(self.close_span())
            }
        }
        impl Fill for $delim_type {
            fn fill(span: Span) -> Self {
                Self::new(span, span)
            }
        }
        impl Desc for $delim_type {
            fn desc() -> &'static str {
                concat!("`", $open_delim, " ", $close_delim, "`")
            }
        }

        impl TokenDowncastFrom<Delimiters> for $delim_type {
            fn downcast_from(value: Delimiters) -> Option<Self> {
                if let Delimiters::$delim_type(value) = value {
                    Some(value)
                } else {
                    None
                }
            }
            fn downcast_from_ref(value: &Delimiters) -> Option<&Self> {
                if let Delimiters::$delim_type(value) = value {
                    Some(value)
                } else {
                    None
                }
            }
        }

        impl $delim_type {
            #[inline(always)]
            pub fn new(open_span: Span, close_span: Span) -> Self {
                Self { open_span, close_span }
            }
        }
    )*
);

with_delimiters!(
    impl Delimiters {$(
        pub fn $delim_fn(open_span: Span, close_span: Span) -> Self {
            Self::$delim_type($delim_type::new(open_span, close_span))
        }
    )*}
);
