use super::*;

#[derive(Debug, Clone, Hash)]
pub struct Group {
    pub delimiters: Delimiters,
    pub tokens: Tokens,
}
impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.delimiters.open_str())?;

        for token in &self.tokens {
            write!(f, " {token}")?;
        }

        write!(f, " {}", self.delimiters.close_str())
    }
}
impl Spanned for Group {
    fn span(&self) -> Span {
        self.delimiters.span()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DelimiterType {
    Braces,
    Brackets,
    Parens,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Delimiters {
    Braces(Braces),
    Brackets(Brackets),
    Parens(Parens),
}
impl Display for Delimiters {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Braces(delimiters) => delimiters.fmt(f),
            Self::Brackets(delimiters) => delimiters.fmt(f),
            Self::Parens(delimiters) => delimiters.fmt(f),
        }
    }
}
impl Spanned for Delimiters {
    fn span(&self) -> Span {
        match self {
            Self::Braces(delimiters) => delimiters.span(),
            Self::Brackets(delimiters) => delimiters.span(),
            Self::Parens(delimiters) => delimiters.span(),
        }
    }
}
impl Delimiters {
    pub fn new(ty: DelimiterType, open_span: Span, close_span: Span) -> Self {
        match ty {
            DelimiterType::Braces => Self::braces(open_span, close_span),
            DelimiterType::Brackets => Self::brackets(open_span, close_span),
            DelimiterType::Parens => Self::parens(open_span, close_span),
        }
    }
    pub fn braces(open_span: Span, close_span: Span) -> Self {
        Self::Braces(Braces::new(open_span, close_span))
    }
    pub fn brackets(open_span: Span, close_span: Span) -> Self {
        Self::Brackets(Brackets::new(open_span, close_span))
    }
    pub fn parens(open_span: Span, close_span: Span) -> Self {
        Self::Parens(Parens::new(open_span, close_span))
    }

    pub fn ty(&self) -> DelimiterType {
        match self {
            Self::Braces(_) => DelimiterType::Braces,
            Self::Brackets(_) => DelimiterType::Brackets,
            Self::Parens(_) => DelimiterType::Parens,
        }
    }

    pub fn open_span(&self) -> Span {
        match self {
            Self::Braces(delimiters) => delimiters.open_span(),
            Self::Brackets(delimiters) => delimiters.open_span(),
            Self::Parens(delimiters) => delimiters.open_span(),
        }
    }
    pub fn close_span(&self) -> Span {
        match self {
            Self::Braces(delimiters) => delimiters.close_span(),
            Self::Brackets(delimiters) => delimiters.close_span(),
            Self::Parens(delimiters) => delimiters.close_span(),
        }
    }
    pub fn set_open_span(&mut self, span: Span) {
        match self {
            Self::Braces(delimiters) => delimiters.set_open_span(span),
            Self::Brackets(delimiters) => delimiters.set_open_span(span),
            Self::Parens(delimiters) => delimiters.set_open_span(span),
        }
    }
    pub fn set_close_span(&mut self, span: Span) {
        match self {
            Self::Braces(delimiters) => delimiters.set_close_span(span),
            Self::Brackets(delimiters) => delimiters.set_close_span(span),
            Self::Parens(delimiters) => delimiters.set_close_span(span),
        }
    }

    pub fn open_str(&self) -> &'static str {
        match self {
            Self::Braces(delimiters) => delimiters.open_str(),
            Self::Brackets(delimiters) => delimiters.open_str(),
            Self::Parens(delimiters) => delimiters.open_str(),
        }
    }
    pub fn close_str(&self) -> &'static str {
        match self {
            Self::Braces(delimiters) => delimiters.close_str(),
            Self::Brackets(delimiters) => delimiters.close_str(),
            Self::Parens(delimiters) => delimiters.close_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Braces {
    open_span: Span,
    close_span: Span,
}
impl Display for Braces {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "{}".fmt(f)
    }
}
impl Spanned for Braces {
    fn span(&self) -> Span {
        self.open_span.join(self.close_span)
    }
}
impl Braces {
    pub fn new(open_span: Span, close_span: Span) -> Self {
        Self {
            open_span,
            close_span,
        }
    }

    pub fn open_span(&self) -> Span {
        self.open_span
    }
    pub fn close_span(&self) -> Span {
        self.close_span
    }
    pub fn set_open_span(&mut self, span: Span) {
        self.open_span = span
    }
    pub fn set_close_span(&mut self, span: Span) {
        self.close_span = span
    }

    pub fn open_str(&self) -> &'static str {
        "{"
    }
    pub fn close_str(&self) -> &'static str {
        "}"
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Brackets {
    open_span: Span,
    close_span: Span,
}
impl Display for Brackets {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "[]".fmt(f)
    }
}
impl Spanned for Brackets {
    fn span(&self) -> Span {
        self.open_span.join(self.close_span)
    }
}
impl Brackets {
    pub fn new(open_span: Span, close_span: Span) -> Self {
        Self {
            open_span,
            close_span,
        }
    }

    pub fn open_span(&self) -> Span {
        self.open_span
    }
    pub fn close_span(&self) -> Span {
        self.close_span
    }
    pub fn set_open_span(&mut self, span: Span) {
        self.open_span = span
    }
    pub fn set_close_span(&mut self, span: Span) {
        self.close_span = span
    }

    pub fn open_str(&self) -> &'static str {
        "["
    }
    pub fn close_str(&self) -> &'static str {
        "]"
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Parens {
    open_span: Span,
    close_span: Span,
}
impl Display for Parens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "()".fmt(f)
    }
}
impl Spanned for Parens {
    fn span(&self) -> Span {
        self.open_span.join(self.close_span)
    }
}
impl Parens {
    pub fn new(open_span: Span, close_span: Span) -> Self {
        Self {
            open_span,
            close_span,
        }
    }

    pub fn open_span(&self) -> Span {
        self.open_span
    }
    pub fn close_span(&self) -> Span {
        self.close_span
    }
    pub fn set_open_span(&mut self, span: Span) {
        self.open_span = span
    }
    pub fn set_close_span(&mut self, span: Span) {
        self.close_span = span
    }

    pub fn open_str(&self) -> &'static str {
        "("
    }
    pub fn close_str(&self) -> &'static str {
        ")"
    }
}
