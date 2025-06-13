use std::mem::{replace, transmute};

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

    fn file(&self) -> FileId;
    fn interner(&self) -> &Interner;
    fn diagnostics(&self) -> &Diagnostics;
    fn highlights(&mut self) -> &mut Vec<Highlight>;
}

#[derive(Debug)]
#[derive(Spanned)]
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

#[derive(Debug)]
pub struct RootTokenizer<'ctx> {
    raw: RawTokenizer<'ctx>,
    peek: Peek<PeekToken>,
    last_span: Span,
}

#[derive(Debug)]
#[derive(Spanned)]
pub struct GroupTokenizer<'ctx, 'parent> {
    parent: ParentTokenizer<'ctx, 'parent>,
    #[span]
    delims: Delimiters,
    peek: Peek<CloseDelimiter>,
    last_span: Span,
}

impl<'ctx> Tokenizer for RootTokenizer<'ctx> {
    fn next(&mut self) -> Option<LazyToken> {
        match replace(&mut self.peek, Peek::Unevaluated) {
            Peek::None => {
                self.peek = Peek::None;

                None
            }
            Peek::Token(token) => Some(match token {
                PeekToken::Ident(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Ident(token)
                }
                PeekToken::Keyword(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Keyword(token)
                }
                PeekToken::Punct(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Punct(token)
                }
                PeekToken::Literal(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Literal(token)
                }
                PeekToken::Group(open) => {
                    let mut group_tokenizer = GroupTokenizer {
                        parent: ParentTokenizer::Root(self),
                        delims: Delimiters::new(open.span, open.span, open.kind, None),
                        peek: Peek::Token(PeekToken::Punct(Punct::new(open.span, PunctKind::And))),
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(unsafe { transmute(group_tokenizer) })
                }
                PeekToken::Error(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Error(token)
                }
            }),
            Peek::Close(token) => {
                self.last_span = token.span();
                self.update_peek();

                Some(LazyToken::Error(match token {
                    PeekToken::Error(error) => error,
                    _ => unreachable!(),
                }))
            }
            Peek::Unevaluated => unreachable!(),
        }
    }

    fn peek(&self) -> Option<&PeekToken> {
        match &self.peek {
            Peek::None => None,
            Peek::Token(token) => Some(token),
            Peek::Unevaluated => unreachable!(),
            Peek::Close(token) => Some(token),
        }
    }
    fn peek_span(&self) -> Span {
        if let Some(next) = self.peek() {
            let span = next.span();

            if span.start().line == self.last_span.end().line {
                span
            } else {
                Span::from_start(self.last_span.end(), 1)
            }
        } else {
            Span::from_start(self.last_span.end(), 1)
        }
    }

    fn file(&self) -> FileId {
        self.raw.file()
    }
    fn interner(&self) -> &Interner {
        self.raw.interner()
    }
    fn diagnostics(&self) -> &Diagnostics {
        self.raw.diagnostics()
    }
    fn highlights(&mut self) -> &mut Vec<Highlight> {
        self.raw.highlights()
    }
}
impl<'ctx> RootTokenizer<'ctx> {
    pub fn new(
        src: &'ctx str,
        file: FileId,
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<Highlight>,
    ) -> Self {
        let mut output = Self {
            raw: RawTokenizer::new(src, file, interner, diagnostics, highlights),
            peek: Peek::Unevaluated,
            last_span: Span::from_start(Position::new(file, 0, 0), 0),
        };

        output.update_peek();

        output
    }

    fn update_peek(&mut self) {
        match &self.peek {
            Peek::None => return,
            Peek::Close(_) => {
                self.peek = Peek::None;
                return;
            }
            Peek::Token(_) => {}
            Peek::Unevaluated => {}
        };

        self.peek = match self.raw.next() {
            None => Peek::None,
            Some(raw_token) => match raw_token {
                RawToken::Ident(raw_token) => Peek::Token(PeekToken::Ident(raw_token)),
                RawToken::Keyword(raw_token) => Peek::Token(PeekToken::Keyword(raw_token)),
                RawToken::Punct(raw_token) => Peek::Token(PeekToken::Punct(raw_token)),
                RawToken::Literal(raw_token) => Peek::Token(PeekToken::Literal(raw_token)),
                RawToken::OpenDelimiter(raw_token) => Peek::Token(PeekToken::Group(raw_token)),
                RawToken::CloseDelimiter(close) => Peek::Close(PeekToken::Error(
                    self.diagnostics()
                        .push_error(TokenError::Unopened(close.span, close.kind.close_str())),
                )),
                RawToken::Unknown(error) => Peek::Token(PeekToken::Error(error)),
            },
        };
    }
}

impl<'ctx, 'parent> Tokenizer for GroupTokenizer<'ctx, 'parent> {
    fn next(&mut self) -> Option<LazyToken> {
        match replace(&mut self.peek, Peek::Unevaluated) {
            Peek::Close(_) | Peek::None => {
                self.peek = Peek::None;

                None
            }
            Peek::Token(token) => Some(match token {
                PeekToken::Ident(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Ident(token)
                }
                PeekToken::Keyword(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Keyword(token)
                }
                PeekToken::Punct(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Punct(token)
                }
                PeekToken::Literal(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Literal(token)
                }
                PeekToken::Group(open) => {
                    let mut group_tokenizer = GroupTokenizer {
                        parent: ParentTokenizer::Group(unsafe { transmute::<&mut GroupTokenizer, &mut GroupTokenizer>(self) }),
                        delims: Delimiters::new(open.span, open.span, open.kind, None),
                        peek: Peek::Token(PeekToken::Punct(Punct::new(open.span, PunctKind::And))),
                        last_span: open.span,
                    };

                    group_tokenizer.update_peek();

                    LazyToken::Group(group_tokenizer)
                }
                PeekToken::Error(token) => {
                    self.last_span = token.span();
                    self.update_peek();

                    LazyToken::Error(token)
                }
            }),
            Peek::Unevaluated => unreachable!(),
        }
    }

    fn peek(&self) -> Option<&PeekToken> {
        match &self.peek {
            Peek::Close(_) | Peek::None => None,
            Peek::Token(token) => Some(token),
            Peek::Unevaluated => unreachable!(),
        }
    }
    fn peek_span(&self) -> Span {
        if let Some(next) = self.peek() {
            let span = next.span();

            if span.start().line == self.last_span.end().line {
                span
            } else {
                Span::from_start(self.last_span.end(), 1)
            }
        } else {
            Span::from_start(self.last_span.end(), 1)
        }
    }

    fn file(&self) -> FileId {
        self.parent.file()
    }
    fn interner(&self) -> &Interner {
        self.parent.interner()
    }
    fn diagnostics(&self) -> &Diagnostics {
        self.parent.diagnostics()
    }
    fn highlights(&mut self) -> &mut Vec<Highlight> {
        self.parent.highlights()
    }
}
impl<'ctx, 'parent> GroupTokenizer<'ctx, 'parent> {
    pub fn open(&self) -> OpenDelimiter {
        self.delims.open()
    }

    pub fn finish(&mut self) -> Delimiters {
        while let Some(_) = self.next() {}

        let errorless_delims = Delimiters::parens(self.delims.open_span, self.delims.close_span, None);
        replace(&mut self.delims, errorless_delims)
    }

    fn update_peek(&mut self) {
        match &self.peek {
            Peek::Token(token) => self.last_span = token.span(),
            Peek::Close(_) | Peek::None => return,
            Peek::Unevaluated => {}
        };

        self.peek = match self.parent.raw_next() {
            None => {
                self.delims.error = Some(
                    self.diagnostics()
                        .push_error(TokenError::Unclosed(self.delims.open_span, self.delims.kind.open_str())),
                );

                match &mut self.parent {
                    ParentTokenizer::Group(parent) => parent.update_peek(),
                    ParentTokenizer::Root(parent) => parent.update_peek(),
                }

                Peek::None
            }
            Some(raw_token) => match raw_token {
                RawToken::Ident(raw_token) => Peek::Token(PeekToken::Ident(raw_token)),
                RawToken::Keyword(raw_token) => Peek::Token(PeekToken::Keyword(raw_token)),
                RawToken::Punct(raw_token) => Peek::Token(PeekToken::Punct(raw_token)),
                RawToken::Literal(raw_token) => Peek::Token(PeekToken::Literal(raw_token)),
                RawToken::OpenDelimiter(raw_token) => Peek::Token(PeekToken::Group(raw_token)),
                RawToken::CloseDelimiter(close) => self.handle_close(close),
                RawToken::Unknown(error) => Peek::Token(PeekToken::Error(error)),
            },
        };
    }

    #[must_use]
    fn handle_close(&mut self, close: CloseDelimiter) -> Peek<CloseDelimiter> {
        Peek::Close(if close.kind == self.open().kind {
            match &mut self.parent {
                ParentTokenizer::Group(parent) => {
                    parent.last_span = close.span;

                    parent.update_peek()
                }
                ParentTokenizer::Root(parent) => {
                    parent.last_span = close.span;

                    parent.update_peek()
                }
            }

            close
        } else {
            self.delims.error = Some(
                self.diagnostics()
                    .push_error(TokenError::Unclosed(self.open().span, self.open().kind.open_str())),
            );

            match &mut self.parent {
                ParentTokenizer::Group(parent) => parent.peek = parent.handle_close(close),
                ParentTokenizer::Root(parent) => {
                    parent.peek = Peek::Close(PeekToken::Error(
                        parent
                            .diagnostics()
                            .push_error(TokenError::Unopened(close.span, close.kind.close_str())),
                    ));
                }
            }

            CloseDelimiter::new(close.span, self.open().kind)
        })
    }
}
impl<'ctx, 'parent> Drop for GroupTokenizer<'ctx, 'parent> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

#[derive(Debug)]
enum Peek<Close> {
    Token(PeekToken),
    Close(Close),
    None,
    Unevaluated,
}

#[derive(Debug)]
enum ParentTokenizer<'ctx, 'parent> {
    Group(&'parent mut GroupTokenizer<'ctx, 'parent>),
    Root(&'parent mut RootTokenizer<'ctx>),
}

impl<'ctx, 'parent> ParentTokenizer<'ctx, 'parent> {
    fn raw_next(&mut self) -> Option<RawToken> {
        match self {
            Self::Root(root) => root.raw.next(),
            Self::Group(group) => group.parent.raw_next(),
        }
    }

    fn file(&self) -> FileId {
        match self {
            Self::Root(root) => root.raw.file(),
            Self::Group(group) => group.parent.file(),
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
    fn highlights(&mut self) -> &mut Vec<Highlight> {
        match self {
            Self::Root(root) => root.raw.highlights(),
            Self::Group(group) => group.parent.highlights(),
        }
    }
}
