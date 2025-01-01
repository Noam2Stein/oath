use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, Spanned};

use crate::Seal;

macro_rules! declare_delimiters {
    ($($delim_ident:ident, )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Delimiters {$(
            $delim_ident($delim_ident),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $delim_ident {
                open_span: Span,
                close_span: Span,
            }
        )*

        #[allow(private_bounds)]
        pub trait DelimitersType: Seal + Send + Sync + Debug + Copy + Eq + Ord + Hash + Spanned {
            fn open_span(self) -> Span;
            fn close_span(self) -> Span;
        }

        impl DelimitersType for Delimiters {
            #[inline(always)]
            fn open_span(self) -> Span {
                match self {$(
                    Self::$delim_ident(delim) => delim.open_span(),
                )*}
            }
            #[inline(always)]
            fn close_span(self) -> Span {
                match self {$(
                    Self::$delim_ident(delim) => delim.close_span(),
                )*}
            }
        }
        impl Seal for Delimiters {}
        impl Spanned for Delimiters {
            #[inline(always)]
            fn span(&self) -> Span {
                self.open_span().connect(self.close_span())
            }
        }

        $(
            impl DelimitersType for $delim_ident {
                #[inline(always)]
                fn open_span(self) -> Span {
                    self.open_span
                }
                #[inline(always)]
                fn close_span(self) -> Span {
                    self.close_span
                }
            }
            impl Spanned for $delim_ident {
                #[inline(always)]
                fn span(&self) -> Span {
                    self.open_span().connect(self.close_span())
                }
            }
            impl Seal for $delim_ident {}
            impl $delim_ident {
                #[inline(always)]
                pub fn new(open_span: Span, close_span: Span) -> Self {
                    Self { open_span, close_span }
                }
            }
        )*
    };
}
declare_delimiters!(Parens, Braces, Brackets,);
