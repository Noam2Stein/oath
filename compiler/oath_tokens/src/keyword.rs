use crate::*;

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

verify_token_type!(Keyword);
with_tokens!(
    $(verify_token_type!($keyword_type);)*
);

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

impl Keyword {
    pub fn from_str(str: &str, span: Span) -> Option<Self> {
        KeywordKind::from_str(str).map(|kind| Self { span, kind })
    }

    pub fn as_str(self) -> &'static str {
        self.kind.as_str()
    }
}
impl KeywordKind {
    pub fn from_str(str: &str) -> Option<Self> {
        with_tokens_expr! {
            match str {
                $($keyword => Some(Self::$keyword_variant),)*
                _ => None,
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        with_tokens_expr! {
            match self {$(
                Self::$keyword_variant => $keyword,
            )*}
        }
    }
}
