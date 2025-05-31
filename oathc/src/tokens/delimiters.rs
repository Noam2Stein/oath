use std::{fmt::Debug, hash::Hash};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[derive(Spanned)]
#[display("`{} {}`", kind.open_str(), kind.close_str())]
pub struct Delimiters {
    pub open_span: Span,
    pub close_span: Try<Span>,
    #[not_spanned]
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

pub trait DelimitersType: Debug + Copy + Spanned + TryFrom<Delimiters> {
    type Open: Debug + Copy + Spanned + TryFrom<OpenDelimiter>;
    type Close: Debug + Copy + Spanned + TryFrom<CloseDelimiter>;

    #[allow(dead_code)]
    fn kind(&self) -> DelimiterKind;

    #[allow(dead_code)]
    fn open(&self) -> Self::Open;
    #[allow(dead_code)]
    fn close(&self) -> Self::Close;
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
            pub open_span: Span,
            pub close_span: Span,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
        #[derive(Spanned)]
        #[display("`{}`", $delim_open)]
        pub struct $delim_open_type(Span);

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
        #[derive(Spanned)]
        #[display("`{}`", $delim_close)]
        pub struct $delim_close_type(Span);
    )*
);

with_tokens!(
    impl Delimiters {
        #[allow(dead_code)]
        pub fn new(open_span: Span, close_span: Span, kind: DelimiterKind) -> Self {
            Self {
                open_span,
                close_span,
                kind,
            }
        }

        $(
            #[allow(dead_code)]
            pub fn $delims_fn(open_span: Span, close_span: Span) -> Self {
                Self::new(open_span, close_span, DelimiterKind::$delims_type)
            }
        )*
    }

    impl OpenDelimiter {
        #[allow(dead_code)]
        pub fn new(span: Span, kind: DelimiterKind) -> Self {
            Self {
                span,
                kind,
            }
        }

        $(
            #[allow(dead_code)]
            pub fn $delim_fn(span: Span) -> Self {
                Self::new(span, DelimiterKind::$delims_type)
            }
        )*
    }

    impl CloseDelimiter {
        #[allow(dead_code)]
        pub fn new(span: Span, kind: DelimiterKind) -> Self {
            Self {
                span,
                kind,
            }
        }

        $(
            #[allow(dead_code)]
            pub fn $delim_fn(span: Span) -> Self {
                Self::new(span, DelimiterKind::$delims_type)
            }
        )*
    }

    $(
        impl $delims_type {
            #[allow(dead_code)]
            pub fn new(open_span: Span, close_span: Span) -> Self {
                Self { open_span, close_span }
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

impl DelimitersType for Delimiters {
    type Open = OpenDelimiter;
    type Close = CloseDelimiter;

    fn kind(&self) -> DelimiterKind {
        self.kind
    }

    fn open(&self) -> Self::Open {
        OpenDelimiter::new(self.open_span, self.kind)
    }
    fn close(&self) -> Self::Close {
        CloseDelimiter::new(self.close_span, self.kind)
    }
}

with_tokens!($(
    impl DelimitersType for $delims_type {
        type Open = $delim_open_type;
        type Close = $delim_close_type;
    
        fn kind(&self) -> DelimiterKind {
            DelimiterKind::$delims_type
        }
    
        fn open(&self) -> Self::Open {
            $delim_open_type(self.open_span)
        }
        fn close(&self) -> Self::Close {
            $delim_close_type(self.close_span)
        }
    }

    impl TryFrom<Delimiters> for $delims_type {
        type Error = ();

        fn try_from(value: Delimiters) -> Result<Self, Self::Error> {
            if value.kind == DelimiterKind::$delims_type {
                Ok(Self {
                    open_span: value.open_span,
                    close_span: value.close_span,
                })
            } else {
                Err(())
            }
        }
    }
    impl TryFrom<OpenDelimiter> for $delim_open_type {
        type Error = ();

        fn try_from(value: OpenDelimiter) -> Result<Self, Self::Error> {
            if value.kind == DelimiterKind::$delims_type {
                Ok(Self(value.span))
            } else {
                Err(())
            }
        }
    }
    impl TryFrom<CloseDelimiter> for $delim_close_type {
        type Error = ();

        fn try_from(value: CloseDelimiter) -> Result<Self, Self::Error> {
            if value.kind == DelimiterKind::$delims_type {
                Ok(Self(value.span))
            } else {
                Err(())
            }
        }
    }
)*);
