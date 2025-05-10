use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new, Spanned)]
#[display("{kind}")]
pub struct Punct {
    #[span]
    pub span: Span,
    pub kind: PunctKind,
}

with_tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
    pub enum PunctKind {$(
        #[display($punct)]
        $punct_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Spanned)]
        #[display($punct)]
        pub struct $punct_type(#[span] pub Span);
    )*
);

verify_token_type!(Punct);
with_tokens!(
    $(verify_token_type!($punct_type);)*
);

pub const PUNCTS: &[&str] = with_tokens_expr! {
    &[$($punct), *]
};

pub fn is_punct(str: &str) -> bool {
    with_tokens_expr! {
        match str {
            $($punct => true,)*
            _ => false,
        }
    }
}

impl Punct {
    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        PunctKind::from_str(str).map(|kind| Self { span, kind })
    }

    pub fn as_str(self) -> &'static str {
        self.kind.as_str()
    }
}
impl PunctKind {
    pub fn from_str(str: &str) -> Option<Self> {
        with_tokens_expr! {
            match str {
                $($punct => Some(Self::$punct_variant),)*
                _ => None,
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        with_tokens_expr! {
            match self {
                $(Self::$punct_variant => $punct,)*
            }
        }
    }
}

impl Highlight for Punct {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight(self.span, color);
    }
}
with_tokens!($(
    impl Highlight for $punct_type {
        fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
            h.highlight(self.0, color);
        }
    }
)*);
