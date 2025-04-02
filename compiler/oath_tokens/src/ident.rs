use crate::*;

#[derive(Debug, Clone, Copy, Hash, Spanned, InternedDisplay)]
#[display("{}", self.str_id)]
pub struct Ident {
    #[span]
    pub span: Span,
    str_id: StrId,
}

verify_token_type!(Ident);

impl Ident {
    pub fn new(str: &str, span: Span, interner: &mut Interner) -> Option<Self> {
        if is_keyword(&str) {
            None
        } else {
            Some(Self {
                str_id: interner.intern(str),
                span,
            })
        }
    }
    pub fn new_adjusted(str: &str, span: Span, interner: &mut Interner) -> Self {
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
    pub unsafe fn new_unchecked(str: &str, span: Span, interner: &mut Interner) -> Self {
        Self {
            str_id: interner.intern(str),
            span,
        }
    }
    pub fn new_or_keyword(str: &str, span: Span, interner: &mut Interner) -> Result<Self, Keyword> {
        if let Some(keyword) = Keyword::from_str(&str, span) {
            Err(keyword)
        } else {
            Ok(Self {
                str_id: interner.intern(str),
                span,
            })
        }
    }

    pub fn str_id(&self) -> StrId {
        self.str_id
    }
}

impl<'a> TryFrom<&'a TokenTree> for Ident {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Ident(value) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}
