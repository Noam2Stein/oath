use super::*;

#[derive(Debug, Clone, Hash)]
pub struct Ident {
    span: Span,
    str: Box<str>,
}
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.str.fmt(f)
    }
}
impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}
impl Ident {
    pub fn new(span: Span, str: impl Into<String>) -> Self {
        Self {
            span,
            str: str.into().into_boxed_str(),
        }
    }
    pub fn str(&self) -> &str {
        &self.str
    }
}
