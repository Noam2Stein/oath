use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Spanned, InternedDisplay)]
#[display("{str_id}")]
pub struct Ident {
    #[span]
    pub span: Span,
    str_id: StrId,
}

verify_token_type!(Ident);

impl Ident {
    pub fn new(str: &str, span: Span, interner: &Interner) -> Option<Self> {
        if is_keyword(&str) {
            None
        } else {
            Some(Self {
                str_id: interner.intern(str),
                span,
            })
        }
    }
    pub fn new_adjusted(str: &str, span: Span, interner: &Interner) -> Self {
        if is_keyword(&str) {
            Self {
                str_id: interner.intern(&format!("@{str}")),
                span,
            }
        } else {
            Self {
                str_id: interner.intern(str),
                span,
            }
        }
    }
    #[inline(always)]
    pub unsafe fn new_unchecked(str: &str, span: Span, interner: &Interner) -> Self {
        Self {
            str_id: interner.intern(str),
            span,
        }
    }
    pub fn new_or_keyword(str: &str, span: Span, interner: &Interner) -> Result<Self, Keyword> {
        if let Some(keyword) = Keyword::from_str(&str, span) {
            Err(keyword)
        } else {
            Ok(Self {
                str_id: interner.intern(str),
                span,
            })
        }
    }

    #[inline(always)]
    pub unsafe fn from_id_unchecked(str_id: StrId, span: Span) -> Self {
        Self { str_id, span }
    }

    pub fn str_id(&self) -> StrId {
        self.str_id
    }
}

impl Highlight for Ident {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight(self.span, color);
    }
}
