use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, SpanLengthed, Spanned};

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
                open_span: SpanLengthed<1>,
                close_span: SpanLengthed<1>,
            }
        )*

        #[allow(private_bounds)]
        pub trait DelimitersType: Seal + Send + Sync + Debug + Copy + Eq + Ord + Hash + Spanned {
            fn open_span(self) -> SpanLengthed<1>;
            fn close_span(self) -> SpanLengthed<1>;
        }

        impl DelimitersType for Delimiters {
            #[inline(always)]
            fn open_span(self) -> SpanLengthed<1> {
                match self {$(
                    Self::$delim_ident(delim) => delim.open_span(),
                )*}
            }
            #[inline(always)]
            fn close_span(self) -> SpanLengthed<1> {
                match self {$(
                    Self::$delim_ident(delim) => delim.close_span(),
                )*}
            }
        }
        impl Seal for Delimiters {}
        impl Spanned for Delimiters {
            #[inline(always)]
            fn span(&self) -> Span {
                self.open_span().unlined().connect(self.close_span().unlined())
            }
        }

        $(
            impl Seal for $delim_ident {}
            impl DelimitersType for $delim_ident {
                #[inline(always)]
                fn open_span(self) -> SpanLengthed<1> {
                    self.open_span
                }
                #[inline(always)]
                fn close_span(self) -> SpanLengthed<1> {
                    self.close_span
                }
            }
            impl Spanned for $delim_ident {
                #[inline(always)]
                fn span(&self) -> Span {
                    self.open_span().unlined().connect(self.close_span().unlined())
                }
            }
            impl $delim_ident {
                #[inline(always)]
                pub fn new(open_span: SpanLengthed<1>, close_span: SpanLengthed<1>) -> Self {
                    Self { open_span, close_span }
                }
            }
        )*
    };
}
declare_delimiters!(Parens, Braces, Brackets,);
