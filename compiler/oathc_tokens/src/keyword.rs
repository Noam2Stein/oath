use super::*;

pub const KEYWORDS: &[&str] = with_tokens_expr! {
    &[$($keyword), *]
};
pub fn is_keyword(str: &str) -> bool {
    with_tokens_expr! {
        match str {
            $($keyword => true,)*
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new, Spanned)]
#[display("{kind}")]
pub struct Keyword {
    #[span]
    pub span: Span,
    pub kind: KeywordKind,
}

with_tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
    pub enum KeywordKind {$(
        #[display($keyword)]
        $keyword_variant,
    )*}

    $(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Spanned)]
        #[display($keyword)]
        pub struct $keyword_type(#[span] pub Span);
    )*
);

const _: () = verify_token_type::<Keyword>();
with_tokens!($(
    const _: () = verify_token_type::<$keyword_type>();
)*);

impl Keyword {
    #[allow(dead_code)]
    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        KeywordKind::from_str(str).map(|kind| Self { span, kind })
    }

    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        self.kind.as_str()
    }
}
impl KeywordKind {
    #[allow(dead_code)]
    pub fn from_str(str: &str) -> Option<Self> {
        with_tokens_expr! {
            match str {
                $($keyword => Some(Self::$keyword_variant),)*
                _ => None,
            }
        }
    }

    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$keyword_variant => $keyword,
            )*}
        }
    }
}

impl Highlightable for Keyword {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        self.span.highlight(color, h);
    }
}
with_tokens! {$(
    impl Highlightable for $keyword_type {
        fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
            self.0.highlight(color, h);
        }
    }
)*}

impl Format for Keyword {
    fn format(&self, _interner: &Interner) -> FormatTree {
        FormatTree::AtomStr(self.as_str())
    }
}
with_tokens! {$(
    impl Format for $keyword_type {
        fn format(&self, _interner: &Interner) -> FormatTree {
            FormatTree::AtomStr($keyword)
        }
    }
)*}
