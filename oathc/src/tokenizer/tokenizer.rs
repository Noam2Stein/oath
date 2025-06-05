use std::{
    mem::{replace, transmute},
    path::Path,
};

use super::*;

//
//
// TOKENIZER
//
//

pub trait Tokenizer {
    fn next(&mut self) -> Option<LazyToken>;

    fn peek(&self) -> Option<&PeekToken>;
    fn peek_span(&self) -> Span;

    fn path(&self) -> &Path;
    fn interner(&self) -> &Interner;
    fn diagnostics(&self) -> &Diagnostics;
    fn highlights(&mut self) -> &mut Vec<HighlightInfo>;
}

//
//
// TOKEN
//
//

#[derive(Debug)]
pub enum LazyToken<'ctx, 'tokenizer> {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(GroupTokenizer<'ctx, 'tokenizer>),
    Error(DiagnosticHandle),
}

#[derive(Debug)]
#[derive(Spanned)]
pub enum PeekToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(OpenDelimiter),
    Error(DiagnosticHandle),
}

//
//
// ROOT TOKENIZER
//
//

#[derive(Debug)]
pub struct RootTokenizer<'ctx> {
    raw: RawTokenizer<'ctx>,
    peek: Option<PeekToken>,
    last_span: Span,
    ended: bool,
}

impl<'ctx> Tokenizer for RootTokenizer<'ctx> {
    fn next(&mut self) -> Option<LazyToken> {
        match replace(&mut self.peek, None) {
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
                        delims: Delimiters::new(open.span, open.span, open.kind, None),
                        peek: GroupPeek::Token(PeekToken::Punct(Punct::new(open.span, PunctKind::And))),
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(unsafe { transmute(group_tokenizer) })
                }
                PeekToken::Error(token) => {
                    self.update_peek();

                    LazyToken::Error(token)
                }
            }),
        }
    }

    fn peek(&self) -> Option<&PeekToken> {
        self.peek.as_ref()
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

    fn path(&self) -> &Path {
        self.raw.path()
    }
    fn interner(&self) -> &Interner {
        self.raw.interner()
    }
    fn diagnostics(&self) -> &Diagnostics {
        self.raw.diagnostics()
    }
    fn highlights(&mut self) -> &mut Vec<HighlightInfo> {
        self.raw.highlights()
    }
}

impl<'ctx> RootTokenizer<'ctx> {
    pub fn new(
        src: &'ctx str,
        path: &'ctx Path,
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<HighlightInfo>,
    ) -> Self {
        let mut output = Self {
            raw: RawTokenizer::new(src, path, interner, diagnostics, highlights),
            peek: Some(PeekToken::Punct(Punct::new(Span::ZERO, PunctKind::And))),
            last_span: Span::ZERO,
            ended: false,
        };

        output.update_peek();

        output
    }

    fn update_peek(&mut self) {
        if self.ended {
            return;
        }

        self.peek = match self.raw.next() {
            None => None,
            Some(raw_token) => match raw_token {
                RawToken::Ident(raw_token) => Some(PeekToken::Ident(raw_token)),
                RawToken::Keyword(raw_token) => Some(PeekToken::Keyword(raw_token)),
                RawToken::Punct(raw_token) => Some(PeekToken::Punct(raw_token)),
                RawToken::Literal(raw_token) => Some(PeekToken::Literal(raw_token)),
                RawToken::OpenDelimiter(raw_token) => Some(PeekToken::Group(raw_token)),
                RawToken::CloseDelimiter(close) => Some(PeekToken::Error(
                    self.diagnostics().push_error(self.path(), TokenError::Unopened(close)),
                )),
                RawToken::Unknown(error) => Some(PeekToken::Error(error)),
            },
        };
    }
}

//
//
// GROUP TOKENIZER
//
//

#[derive(Debug)]
pub struct GroupTokenizer<'ctx, 'parent> {
    parent: ParentTokenizer<'ctx, 'parent>,
    delims: Delimiters,
    peek: GroupPeek,
    last_span: Span,
}

#[derive(Debug)]
enum GroupPeek {
    Token(PeekToken),
    Close(CloseDelimiter),
    Unevaluated,
}

#[derive(Debug)]
enum ParentTokenizer<'ctx, 'parent> {
    Group(&'parent mut GroupTokenizer<'ctx, 'parent>),
    Root(&'parent mut RootTokenizer<'ctx>),
}

impl<'ctx, 'parent> Tokenizer for GroupTokenizer<'ctx, 'parent> {
    fn next(&mut self) -> Option<LazyToken> {
        match replace(&mut self.peek, GroupPeek::Unevaluated) {
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
                        delims: Delimiters::new(open.span, open.span, open.kind, None),
                        peek: GroupPeek::Token(PeekToken::Punct(Punct::new(open.span, PunctKind::And))),
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(group_tokenizer)
                }
                PeekToken::Error(token) => {
                    self.update_peek();

                    LazyToken::Error(token)
                }
            }),
            GroupPeek::Unevaluated => unreachable!(),
        }
    }

    fn peek(&self) -> Option<&PeekToken> {
        match &self.peek {
            GroupPeek::Close(_) => None,
            GroupPeek::Token(token) => Some(token),
            GroupPeek::Unevaluated => unreachable!(),
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

    fn path(&self) -> &Path {
        self.parent.path()
    }
    fn interner(&self) -> &Interner {
        self.parent.interner()
    }
    fn diagnostics(&self) -> &Diagnostics {
        self.parent.diagnostics()
    }
    fn highlights(&mut self) -> &mut Vec<HighlightInfo> {
        self.parent.highlights()
    }
}

impl<'ctx, 'parent> Drop for GroupTokenizer<'ctx, 'parent> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<'ctx, 'parent> GroupTokenizer<'ctx, 'parent> {
    pub fn open(&self) -> OpenDelimiter {
        self.delims.open()
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
                GroupPeek::Unevaluated => unreachable!(),
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
                let open = self.open;
                self.context().push_error(TokenError::Unclosed(open));

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
                ParentTokenizer::Group(parent) => {
                    parent.last_span = close.span;
                    match &mut parent.peek {
                        GroupPeek::Token(PeekToken::Group(group)) => group.span = close.span,
                        _ => unreachable!(),
                    }

                    parent.update_peek()
                }
                ParentTokenizer::Root(parent) => {
                    parent.last_span = close.span;
                    match &mut parent.peek {
                        Some(PeekToken::Group(group)) => group.span = close.span,
                        _ => unreachable!(),
                    }

                    parent.update_peek()
                }
            }

            close
        } else {
            let open = self.open;
            self.context().push_error(TokenError::Unclosed(open));

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

impl<'ctx, 'parent> ParentTokenizer<'ctx, 'parent> {
    fn raw_next(&mut self) -> Option<RawToken> {
        match self {
            Self::Root(root) => root.raw.next(),
            Self::Group(group) => group.parent.raw_next(),
        }
    }

    fn path(&self) -> &'ctx Path {
        match self {
            Self::Root(root) => root.raw.path(),
            Self::Group(group) => group.parent.path(),
        }
    }
    fn interner(&self) -> &'ctx Interner {
        match self {
            Self::Root(root) => root.raw.interner(),
            Self::Group(group) => group.parent.interner(),
        }
    }
    fn diagnostics(&self) -> &'ctx Diagnostics {
        match self {
            Self::Root(root) => root.raw.diagnostics(),
            Self::Group(group) => group.parent.diagnostics(),
        }
    }
    fn highlights(&mut self) -> &mut Vec<HighlightInfo> {
        match self {
            Self::Root(root) => root.raw.highlights(),
            Self::Group(group) => group.parent.highlights(),
        }
    }
}
