use std::{fmt::Debug, hash::Hash};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[derive(Spanned)]
#[display("`{} {}`", kind.open_str(), kind.close_str())]
pub struct Delimiters {
    #[span]
    pub open_span: Span,
    #[span]
    pub close_span: Span,
    pub kind: DelimiterKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[derive(Spanned)]
#[display("`{}`", kind.open_str())]
pub struct OpenDelimiter {
    #[span]
    pub span: Span,
    pub kind: DelimiterKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[derive(Spanned)]
#[display("`{}`", kind.close_str())]
pub struct CloseDelimiter {
    #[span]
    pub span: Span,
    pub kind: DelimiterKind,
}

with_tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DelimiterKind {$(
        $delims_type,
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
        #[derive(Spanned)]
        #[display("`{} {}`", $delim_open, $delim_close)]
        pub struct $delims_type {
            #[span]
            pub open_span: Span,
            #[span]
            pub close_span: Span,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
        #[derive(Spanned)]
        #[display("`{}`", $delim_open)]
        pub struct $delim_open_type {
            #[span]
            pub span: Span,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
        #[derive(Spanned)]
        #[display("`{}`", $delim_close)]
        pub struct $delim_close_type {
            #[span]
            pub span: Span,
        }
    )*
);

with_tokens!(
    impl Delimiters {
        pub fn new(open_span: Span, close_span: Span, kind: DelimiterKind) -> Self {
            Self {
                open_span,
                close_span,
                kind,
            }
        }

        $(
            pub fn $delims_fn(open_span: Span, close_span: Span) -> Self {
                Self::new(open_span, close_span, DelimiterKind::$delims_type)
            }
        )*
    }

    impl OpenDelimiter {
        pub fn new(span: Span, kind: DelimiterKind) -> Self {
            Self {
                span,
                kind,
            }
        }

        $(
            pub fn $delim_fn(span: Span) -> Self {
                Self::new(span, DelimiterKind::$delims_type)
            }
        )*
    }

    impl CloseDelimiter {
        pub fn new(span: Span, kind: DelimiterKind) -> Self {
            Self {
                span,
                kind,
            }
        }

        $(
            pub fn $delim_fn(span: Span) -> Self {
                Self::new(span, DelimiterKind::$delims_type)
            }
        )*
    }

    $(
        impl $delims_type {
            #[inline(always)]
            pub fn new(open_span: Span, close_span: Span) -> Self {
                Self { open_span, close_span }
            }
        }

        impl $delim_open_type {
            #[inline(always)]
            pub fn new(span: Span) -> Self {
                Self { span }
            }
        }

        impl $delim_close_type {
            #[inline(always)]
            pub fn new(span: Span) -> Self {
                Self { span }
            }
        }
    )*
);

impl DelimiterKind {
    pub fn open_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$delims_type => $delim_open,
            )*}
        }
    }
    pub fn close_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$delims_type => $delim_close,
            )*}
        }
    }
}
