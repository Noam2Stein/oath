use std::mem::transmute;

use super::*;

//
//
// TOKENIZER
//
//

pub trait Tokenizer<'src, 'ctx> {
    fn next(&mut self) -> Option<LazyToken<'src, 'ctx, '_>>;

    fn peek(&self) -> Option<PeekToken>;
    fn peek_span(&self) -> Span;

    fn context(&self) -> ContextHandle<'ctx>;
}

//
//
// TOKEN
//
//

pub enum LazyToken<'src, 'ctx, 'tokenizer> {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(GroupTokenizer<'src, 'ctx, 'tokenizer>),
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
// ROOT TOKENIZER
//
//

pub struct RootTokenizer<'src, 'ctx> {
    raw: RawTokenizer<'src, 'ctx>,
    peek: Option<PeekToken>,
    last_span: Span,
}

impl<'src, 'ctx> Tokenizer<'src, 'ctx> for RootTokenizer<'src, 'ctx> {
    fn next(&mut self) -> Option<LazyToken<'src, 'ctx, '_>> {
        match self.peek {
            None => None,
            Some(token) => Some(match token {
                PeekToken::Ident(token) => {
                    self.update_peek();

                    LazyToken::Ident(token)
                }
                PeekToken::Keyword(token) => {
                    self.update_peek();

                    LazyToken::Keyword(token)
                }
                PeekToken::Punct(token) => {
                    self.update_peek();

                    LazyToken::Punct(token)
                }
                PeekToken::Literal(token) => {
                    self.update_peek();

                    LazyToken::Literal(token)
                }
                PeekToken::Group(group_open) => {
                    let mut group_tokenizer = GroupTokenizer {
                        parent: ParentTokenizer::Root(self),
                        open: group_open,
                        close: None,
                        peek: None,
                        last_span: group_open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(group_tokenizer)
                }
            }),
        }
    }

    fn peek(&self) -> Option<PeekToken> {
        self.peek
    }
    fn peek_span(&self) -> Span {
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

    fn context(&self) -> ContextHandle<'ctx> {
        self.raw.context()
    }
}

impl<'src, 'ctx> RootTokenizer<'src, 'ctx> {
    pub fn new(src: &'src SrcFile, context: ContextHandle<'ctx>) -> Self {
        let mut output = Self {
            raw: RawTokenizer::new(src.as_str(), context),
            peek: None,
            last_span: Span::ZERO,
        };

        output.update_peek();

        output
    }

    fn update_peek(&mut self) {
        self.peek = match self.raw.next() {
            None => None,
            Some(raw_token) => Some(match raw_token {
                RawToken::Ident(raw_token) => PeekToken::Ident(raw_token),
                RawToken::Keyword(raw_token) => PeekToken::Keyword(raw_token),
                RawToken::Punct(raw_token) => PeekToken::Punct(raw_token),
                RawToken::Literal(raw_token) => PeekToken::Literal(raw_token),
                RawToken::OpenDelimiter(raw_token) => PeekToken::Group(raw_token),
                RawToken::CloseDelimiter(close) => {
                    self.context().push_error(TokenError::Unopened(close));

                    return;
                }
            }),
        };
    }
}

//
//
// GROUP TOKENIZER
//
//

pub struct GroupTokenizer<'src, 'ctx, 'parent> {
    parent: ParentTokenizer<'src, 'ctx, 'parent>,
    open: OpenDelimiter,
    close: Option<CloseDelimiter>,
    peek: Option<PeekToken>,
    last_span: Span,
}

enum ParentTokenizer<'src, 'ctx, 'parent> {
    Group(&'parent mut GroupTokenizer<'src, 'ctx, 'parent>),
    Root(&'parent mut RootTokenizer<'src, 'ctx>),
}

impl<'src, 'ctx, 'parent> Tokenizer<'src, 'ctx> for GroupTokenizer<'src, 'ctx, 'parent> {
    fn next(&mut self) -> Option<LazyToken<'src, 'ctx, '_>> {
        match self.peek {
            None => None,
            Some(token) => Some(match token {
                PeekToken::Ident(token) => {
                    self.update_peek();

                    LazyToken::Ident(token)
                }
                PeekToken::Keyword(token) => {
                    self.update_peek();

                    LazyToken::Keyword(token)
                }
                PeekToken::Punct(token) => {
                    self.update_peek();

                    LazyToken::Punct(token)
                }
                PeekToken::Literal(token) => {
                    self.update_peek();

                    LazyToken::Literal(token)
                }
                PeekToken::Group(open) => {
                    let mut group_tokenizer = GroupTokenizer {
                        parent: ParentTokenizer::Group(unsafe { transmute(self) }),
                        open,
                        close: None,
                        peek: None,
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(group_tokenizer)
                }
            }),
        }
    }

    fn peek(&self) -> Option<PeekToken> {
        self.peek
    }
    fn peek_span(&self) -> Span {
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

    fn context(&self) -> ContextHandle<'ctx> {
        match &self.parent {
            ParentTokenizer::Root(parent) => parent.context(),
            ParentTokenizer::Group(parent) => parent.context(),
        }
    }
}

impl<'src, 'ctx, 'parent> Drop for GroupTokenizer<'src, 'ctx, 'parent> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<'src, 'ctx, 'parent> GroupTokenizer<'src, 'ctx, 'parent> {
    pub fn open(&self) -> OpenDelimiter {
        self.open
    }
    pub fn close(&mut self) -> CloseDelimiter {
        while self.peek().is_some() {
            self.next();
        }

        self.close.unwrap()
    }

    pub fn delims(&mut self) -> Delimiters {
        Delimiters::new(self.open().span(), self.close().span(), self.open().kind)
    }

    fn update_peek(&mut self) {
        if self.close.is_some() {
            return;
        }

        self.peek = match self.parent.raw_next() {
            None => None,
            Some(raw_token) => match raw_token {
                RawToken::Ident(raw_token) => Some(PeekToken::Ident(raw_token)),
                RawToken::Keyword(raw_token) => Some(PeekToken::Keyword(raw_token)),
                RawToken::Punct(raw_token) => Some(PeekToken::Punct(raw_token)),
                RawToken::Literal(raw_token) => Some(PeekToken::Literal(raw_token)),
                RawToken::OpenDelimiter(raw_token) => Some(PeekToken::Group(raw_token)),
                RawToken::CloseDelimiter(close) => {
                    self.handle_close(close);

                    None
                }
            },
        };
    }

    fn handle_close(&mut self, close: CloseDelimiter) {
        if close.kind == self.open.kind {
            self.close = Some(close);

            match &mut self.parent {
                ParentTokenizer::Group(parent) => parent.update_peek(),
                ParentTokenizer::Root(parent) => parent.update_peek(),
            }
        } else {
            self.context().push_error(TokenError::Unclosed(self.open));

            match &mut self.parent {
                ParentTokenizer::Group(parent) => parent.handle_close(close),
                ParentTokenizer::Root(_) => self.context().push_error(TokenError::Unopened(close)),
            }
        };
    }
}

impl<'src, 'ctx, 'parent> ParentTokenizer<'src, 'ctx, 'parent> {
    fn raw_next(&mut self) -> Option<RawToken> {
        match self {
            Self::Root(root) => root.raw.next(),
            Self::Group(group) => group.parent.raw_next(),
        }
    }
}
