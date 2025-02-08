use std::{fmt::Debug, hash::Hash};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Delimiters {
    open_span: Span,
    close_span: Span,
    pub kind: DelimiterKind,
}

with_token_set!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DelimiterKind {$(
        $delim_type,
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
    Seal + Send + Sync + Debug + Copy + Eq + Ord + Hash + Spanned + TryFrom<Delimiters>
{
    fn open_span(self) -> Span;
    fn close_span(self) -> Span;
}

impl DelimitersType for Delimiters {
    #[inline(always)]
    fn open_span(self) -> Span {
        self.open_span
    }
    #[inline(always)]
    fn close_span(self) -> Span {
        self.close_span
    }
}
impl Seal for Delimiters {}

impl Spanned for Delimiters {
    #[inline(always)]
    fn span(&self) -> Span {
        self.open_span().connect(self.close_span())
    }
}

with_token_set!(
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

        impl TryFrom<Delimiters> for $delim_type {
            type Error = ();

            fn try_from(value: Delimiters) -> Result<Self, Self::Error> {
                if value.kind == DelimiterKind::$delim_type {
                    Ok(Self {
                        open_span: value.open_span,
                        close_span: value.close_span,
                    })
                } else {
                    Err(())
                }
            }
        }

        impl Spanned for $delim_type {
            #[inline(always)]
            fn span(&self) -> Span {
                self.open_span().connect(self.close_span())
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

impl Delimiters {
    pub fn new(open_span: Span, close_span: Span, kind: DelimiterKind) -> Self {
        Self {
            open_span,
            close_span,
            kind,
        }
    }
}

with_token_set!(
    impl Delimiters {$(
        pub fn $delim_fn(open_span: Span, close_span: Span) -> Self {
            Self::new(open_span, close_span, DelimiterKind::$delim_type)
        }
    )*}
);

impl DelimiterKind {
    pub fn open_str(self) -> &'static str {
        with_token_set_expr! {
            match self {$(
                Self::$delim_type => $delim_open,
            )*}
        }
    }
    pub fn close_str(self) -> &'static str {
        with_token_set_expr! {
            match self {$(
                Self::$delim_type => $delim_close,
            )*}
        }
    }
}
