use std::{fmt::Debug, hash::Hash};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Delimiters {
    pub open_span: Span,
    pub close_span: Span,
    pub kind: DelimiterKind,
}

with_tokens!(
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
    DelimitersTypeSeal
    + Sized
    + Send
    + Sync
    + Debug
    + Copy
    + Eq
    + Ord
    + Hash
    + Spanned
    + Into<Delimiters>
    + TryFrom<Delimiters>
{
    const GROUP_DESC: &str;

    fn open_span(&self) -> Span;
    fn close_span(&self) -> Span;

    fn kind(&self) -> DelimiterKind;
}
trait DelimitersTypeSeal {}

impl DelimitersType for Delimiters {
    const GROUP_DESC: &str = "expected a group";

    #[inline(always)]
    fn open_span(&self) -> Span {
        self.open_span
    }
    #[inline(always)]
    fn close_span(&self) -> Span {
        self.close_span
    }

    fn kind(&self) -> DelimiterKind {
        self.kind
    }
}
impl DelimitersTypeSeal for Delimiters {}

impl Spanned for Delimiters {
    #[inline(always)]
    fn span(&self) -> Span {
        self.open_span() + self.close_span()
    }
}

with_tokens!(
    $(
        impl DelimitersType for $delim_type {
            const GROUP_DESC: &str = concat!("expected `", $delim_open, " ", $delim_close, "`");

            fn open_span(&self) -> Span {
                self.open_span
            }
            fn close_span(&self) -> Span {
                self.close_span
            }

            fn kind(&self) -> DelimiterKind {
                DelimiterKind::$delim_type
            }
        }
        impl DelimitersTypeSeal for $delim_type {}

        impl From<$delim_type> for Delimiters {
            fn from(value: $delim_type) -> Self {
                Self {
                    open_span: value.open_span,
                    close_span: value.close_span,
                    kind: DelimiterKind::$delim_type,
                }
            }
        }
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
                self.open_span() + self.close_span()
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

with_tokens!(
    impl Delimiters {$(
        pub fn $delim_fn(open_span: Span, close_span: Span) -> Self {
            Self::new(open_span, close_span, DelimiterKind::$delim_type)
        }
    )*}
);

impl DelimiterKind {
    pub fn open_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$delim_type => $delim_open,
            )*}
        }
    }
    pub fn close_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$delim_type => $delim_close,
            )*}
        }
    }
}
