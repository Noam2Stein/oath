use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Spanned, InternedDisplay)]
#[display("{str_id}")]
pub struct Ident {
    #[span]
    pub span: Span,
    str_id: StrId,
}

const _: () = verify_token_type::<Ident>();

impl Ident {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub unsafe fn new_unchecked(str: &str, span: Span, interner: &Interner) -> Self {
        Self {
            str_id: interner.intern(str),
            span,
        }
    }
    #[allow(dead_code)]
    pub unsafe fn raw_new_unchecked(str_id: StrId, span: Span) -> Self {
        Self { str_id, span }
    }

    #[allow(dead_code)]
    pub fn str_id(&self) -> StrId {
        self.str_id
    }
}

impl Highlightable for Ident {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        self.span.highlight(color, h);
    }
}

impl Format for Ident {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
