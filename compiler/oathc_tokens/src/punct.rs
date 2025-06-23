use super::*;

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
        #[allow(dead_code)]
        #[display($punct)]
        $punct_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Spanned)]
        #[display($punct)]
        pub struct $punct_type(#[span] pub Span);
    )*
);

const _: () = verify_token_type::<Punct>();
with_tokens!($(
    const _: () = verify_token_type::<$punct_type>();
)*);

impl Punct {
    #[allow(dead_code)]
    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        PunctKind::from_str(str).map(|kind| Self { span, kind })
    }

    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        self.kind.as_str()
    }
}
impl PunctKind {
    #[allow(dead_code)]
    pub fn from_str(str: &str) -> Option<Self> {
        with_tokens_expr! {
            match str {
                $($punct => Some(Self::$punct_variant),)*
                _ => None,
            }
        }
    }

    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        with_tokens_expr! {
            match self {
                $(Self::$punct_variant => $punct,)*
            }
        }
    }
}

impl Highlightable for Punct {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        self.span.highlight(color, h);
    }
}
with_tokens! {$(
    impl Highlightable for $punct_type {
        fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
            self.0.highlight(color, h);
        }
    }
)*}

impl Format for Punct {
    fn format(&self, _interner: &Interner) -> FormatTree {
        FormatTree::AtomStr(self.as_str())
    }
}
with_tokens! {$(
    impl Format for $punct_type {
        fn format(&self, _interner: &Interner) -> FormatTree {
            FormatTree::AtomStr($punct)
        }
    }
)*}
