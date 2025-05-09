use super::*;

//
//
// TOKENIZER
//
//

pub struct Tokenizer<Src: TokenSource> {
    src: Src,
    next: Option<PeekToken>,
    last_span: Span,
}

impl<Src: TokenSource> Tokenizer<Src> {
    pub fn next(&mut self) -> Option<LazyToken<Src>> {
        match self.next {
            None => None,
            Some(token) => Some(match token {
                PeekToken::Ident(token) => {
                    self.update_next();
                    LazyToken::Ident(token)
                }
                PeekToken::Keyword(token) => {
                    self.update_next();
                    LazyToken::Keyword(token)
                }
                PeekToken::Punct(token) => {
                    self.update_next();
                    LazyToken::Punct(token)
                }
                PeekToken::Literal(token) => {
                    self.update_next();
                    LazyToken::Literal(token)
                }
                PeekToken::Group(group_open) => {
                    let mut group_tokenizer = Box::new(Tokenizer {
                        src: GroupSource {
                            open: group_open,
                            parent: self,
                            close: None,
                        },
                        next: None,
                        last_span: group_open.span,
                    });

                    group_tokenizer.update_next();

                    LazyToken::Group(group_tokenizer)
                }
            }),
        }
    }
    pub fn peek(&self) -> Option<PeekToken> {
        self.next
    }

    pub fn peek_span(&self) -> Span {
        if let Some(next) = self.peek() {
            let span = next.span();

            if span.start().line == self.last_span.end().line {
                span
            } else {
                Span::from_start_len(self.last_span.end(), 1)
            }
        } else {
            Span::from_start_len(self.last_span.end(), 1)
        }
    }
    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
    pub fn is_not_empty(&self) -> bool {
        self.peek().is_some()
    }

    pub fn context(&self) -> ContextHandle {
        self.src.context()
    }

    fn update_next(&mut self) {
        if self.next.is_none() {
            return;
        }

        self.next = match self.src.raw_next() {
            None => None,
            Some(raw_token) => Some(match raw_token {
                RawToken::Ident(raw_token) => PeekToken::Ident(raw_token),
                RawToken::Keyword(raw_token) => PeekToken::Keyword(raw_token),
                RawToken::Punct(raw_token) => PeekToken::Punct(raw_token),
                RawToken::Literal(raw_token) => PeekToken::Literal(raw_token),
                RawToken::OpenDelimiter(raw_token) => PeekToken::Group(raw_token),
                RawToken::CloseDelimiter(close) => {
                    self.close_end(close);

                    return;
                }
            }),
        };
    }

    fn close_end(&mut self, close: CloseDelimiter) {
        self.next = None;

        self.src.close_end(close);
    }
}
impl<Src: TokenSource> Drop for Tokenizer<Src> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<'ctx, 'src> Tokenizer<RootSource<'src, 'ctx>> {
    pub fn new(src: &'src SrcFile, context: ContextHandle<'ctx>) -> Self {
        let mut output = Self {
            src: RootSource {
                raw: RawTokenizer::new(src.as_str(), context),
            },
            next: None,
            last_span: Span::ZERO,
        };

        output.update_next();

        output
    }
}

impl<'parent, D: DelimitersType, ParentSrc: TokenSource> Tokenizer<GroupSource<'parent, D, ParentSrc>> {
    pub fn open(&self) -> D::Open {
        self.src.open
    }

    pub fn close(&mut self) -> D::Close {
        while self.peek().is_some() {
            self.next();
        }

        self.src.close.unwrap()
    }

    pub fn delims(&mut self) -> D {
        Delimiters::new(self.open().span(), self.close().span(), self.)
    }
}

//
//
// TOKEN
//
//

pub enum LazyToken<'tokenizer, Src: TokenSource> {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(Box<Tokenizer<GroupSource<'tokenizer, Delimiters, Src>>>),
}

#[derive(Debug, Clone, Copy, Hash)]
#[derive(Spanned)]
pub enum PeekToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(OpenDelimiter),
}

//
//
// TOKEN SRC
//
//

#[allow(private_bounds)]
pub trait TokenSource: TokenSourcePrivate {}
trait TokenSourcePrivate {
    fn context(&self) -> ContextHandle;

    fn raw_next(&mut self) -> Option<RawToken>;

    fn close_end(&mut self, close: CloseDelimiter);
}

pub struct RootSource<'src, 'ctx> {
    raw: RawTokenizer<'src, 'ctx>,
}

pub struct GroupSource<'parent, D: DelimitersType, ParentSrc: TokenSource> {
    open: D::Open,
    parent: &'parent mut Tokenizer<ParentSrc>,
    close: Option<D::Close>,
}

impl<'src, 'ctx> TokenSource for RootSource<'src, 'ctx> {}
impl<'src, 'ctx> TokenSourcePrivate for RootSource<'src, 'ctx> {
    fn context(&self) -> ContextHandle {
        self.raw.context()
    }

    fn raw_next(&mut self) -> Option<RawToken> {
        self.raw.next()
    }

    fn close_end(&mut self, close: CloseDelimiter) {
        self.context().push_error(TokenError::Unopened(close));
    }
}

impl<'parent, D: DelimitersType, ParentSrc: TokenSource> TokenSource for GroupSource<'parent, D, ParentSrc> {}
impl<'parent, D: DelimitersType, ParentSrc: TokenSource> TokenSourcePrivate for GroupSource<'parent, D, ParentSrc> {
    fn context(&self) -> ContextHandle {
        self.parent.context()
    }

    fn raw_next(&mut self) -> Option<RawToken> {
        self.parent.src.raw_next()
    }

    fn close_end(&mut self, close: CloseDelimiter) {
        if let Ok(close) = close.try_into() {
            self.close = Some(close);
            self.parent.update_next();
        } else {
            self.parent.close_end(close);
        }
    }
}
