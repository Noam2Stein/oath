use std::{mem::transmute, sync::Arc};

use super::*;

//
//
// TOKENIZER
//
//

pub trait Tokenizer {
    fn next(&mut self) -> Option<LazyToken>;

    fn peek(&self) -> Option<PeekToken>;
    fn peek_span(&self) -> Span;

    fn context(&self) -> &Arc<Context>;
}

//
//
// TOKEN
//
//

pub enum LazyToken<'src, 'tokenizer> {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(GroupTokenizer<'src, 'tokenizer>),
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

pub struct RootTokenizer<'src> {
    raw: RawTokenizer<'src>,
    peek: Option<PeekToken>,
    last_span: Span,
}

impl<'src> Tokenizer for RootTokenizer<'src> {
    fn next(&mut self) -> Option<LazyToken> {
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
                        parent: ParentTokenizer::Root(self),
                        open,
                        peek: GroupPeek::Token(PeekToken::Punct(Punct::new(open.span, PunctKind::And))),
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(unsafe { transmute(group_tokenizer) })
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

    fn context(&self) -> &Arc<Context> {
        self.raw.context()
    }
}

impl<'src> RootTokenizer<'src> {
    pub fn new(src: &'src SrcFile, context: Arc<Context>) -> Self {
        let mut output = Self {
            raw: RawTokenizer::new(src.as_str(), context),
            peek: Some(PeekToken::Punct(Punct::new(Span::ZERO, PunctKind::And))),
            last_span: Span::ZERO,
        };

        output.update_peek();

        output
    }

    fn update_peek(&mut self) {
        match self.peek {
            Some(token) => self.last_span = token.span(),
            None => return,
        }

        self.peek = match self.raw.next() {
            None => None,
            Some(raw_token) => match raw_token {
                RawToken::Ident(raw_token) => Some(PeekToken::Ident(raw_token)),
                RawToken::Keyword(raw_token) => Some(PeekToken::Keyword(raw_token)),
                RawToken::Punct(raw_token) => Some(PeekToken::Punct(raw_token)),
                RawToken::Literal(raw_token) => Some(PeekToken::Literal(raw_token)),
                RawToken::OpenDelimiter(raw_token) => Some(PeekToken::Group(raw_token)),
                RawToken::CloseDelimiter(close) => {
                    self.context().push_error(TokenError::Unopened(close));

                    self.update_peek();

                    return;
                }
            },
        };
    }
}

//
//
// GROUP TOKENIZER
//
//

pub struct GroupTokenizer<'src, 'parent> {
    parent: ParentTokenizer<'src, 'parent>,
    open: OpenDelimiter,
    peek: GroupPeek,
    last_span: Span,
}

enum GroupPeek {
    Token(PeekToken),
    Close(CloseDelimiter),
}

enum ParentTokenizer<'src, 'parent> {
    Group(&'parent mut GroupTokenizer<'src, 'parent>),
    Root(&'parent mut RootTokenizer<'src>),
}

impl<'src, 'parent> Tokenizer for GroupTokenizer<'src, 'parent> {
    fn next(&mut self) -> Option<LazyToken> {
        match self.peek {
            GroupPeek::Close(_) => None,
            GroupPeek::Token(token) => Some(match token {
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
                        peek: GroupPeek::Token(PeekToken::Punct(Punct::new(open.span, PunctKind::And))),
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(group_tokenizer)
                }
            }),
        }
    }

    fn peek(&self) -> Option<PeekToken> {
        match self.peek {
            GroupPeek::Close(_) => None,
            GroupPeek::Token(token) => Some(token),
        }
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

    fn context(&self) -> &Arc<Context> {
        match &self.parent {
            ParentTokenizer::Root(parent) => parent.context(),
            ParentTokenizer::Group(parent) => parent.context(),
        }
    }
}

impl<'src, 'parent> Drop for GroupTokenizer<'src, 'parent> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<'src, 'parent> GroupTokenizer<'src, 'parent> {
    pub fn open(&self) -> OpenDelimiter {
        self.open
    }
    pub fn close(&mut self) -> CloseDelimiter {
        loop {
            match self.peek {
                GroupPeek::Token(_) => {
                    self.next();
                }
                GroupPeek::Close(close) => {
                    return close;
                }
            }
        }
    }

    pub fn delims(&mut self) -> Delimiters {
        Delimiters::new(self.open().span(), self.close().span(), self.open().kind)
    }

    fn update_peek(&mut self) {
        match self.peek {
            GroupPeek::Token(token) => self.last_span = token.span(),
            GroupPeek::Close(_) => return,
        }

        self.peek = match self.parent.raw_next() {
            None => {
                self.context().push_error(TokenError::Unclosed(self.open));

                match &mut self.parent {
                    ParentTokenizer::Group(parent) => parent.update_peek(),
                    ParentTokenizer::Root(parent) => parent.update_peek(),
                }

                GroupPeek::Close(CloseDelimiter::new(self.last_span, self.open.kind))
            }
            Some(raw_token) => match raw_token {
                RawToken::Ident(raw_token) => GroupPeek::Token(PeekToken::Ident(raw_token)),
                RawToken::Keyword(raw_token) => GroupPeek::Token(PeekToken::Keyword(raw_token)),
                RawToken::Punct(raw_token) => GroupPeek::Token(PeekToken::Punct(raw_token)),
                RawToken::Literal(raw_token) => GroupPeek::Token(PeekToken::Literal(raw_token)),
                RawToken::OpenDelimiter(raw_token) => GroupPeek::Token(PeekToken::Group(raw_token)),
                RawToken::CloseDelimiter(close) => self.handle_close(close),
            },
        };
    }

    fn handle_close(&mut self, close: CloseDelimiter) -> GroupPeek {
        GroupPeek::Close(if close.kind == self.open.kind {
            match &mut self.parent {
                ParentTokenizer::Group(parent) => parent.update_peek(),
                ParentTokenizer::Root(parent) => parent.update_peek(),
            }

            close
        } else {
            self.context().push_error(TokenError::Unclosed(self.open));

            match &mut self.parent {
                ParentTokenizer::Group(parent) => parent.peek = parent.handle_close(close),
                ParentTokenizer::Root(parent) => {
                    parent.context().push_error(TokenError::Unopened(close));

                    parent.update_peek();
                }
            }

            CloseDelimiter::new(close.span, self.open.kind)
        })
    }
}

impl<'src, 'parent> ParentTokenizer<'src, 'parent> {
    fn raw_next(&mut self) -> Option<RawToken> {
        match self {
            Self::Root(root) => root.raw.next(),
            Self::Group(group) => group.parent.raw_next(),
        }
    }
}
