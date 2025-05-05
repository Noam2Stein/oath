use std::mem::transmute;

use super::*;

pub struct Tokenizer<'src, 'ctx, 'parent> {
    kind: TokenizerKind<'src, 'ctx, 'parent>,
    context: ContextHandle<'ctx>,
    next: Option<PeekToken>,
}

#[derive(Spanned)]
pub enum LazyToken<'src, 'ctx, 'lexer> {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    Group(#[span] OpenDelimiter, Box<Tokenizer<'src, 'ctx, 'lexer>>),
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

impl<'src, 'ctx, 'lexer> Tokenizer<'src, 'ctx, 'lexer> {
    pub fn new(src: &'src SrcFile, context: ContextHandle<'ctx>) -> Self {
        let mut output = Self {
            kind: TokenizerKind::Root(RawTokenizer::new(src.as_str(), context)),
            next: None,
            context,
        };

        output.update_next();

        output
    }

    pub fn next(&mut self) -> Option<LazyToken<'src, 'ctx, '_>> {
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
                        context: self.context,
                        kind: TokenizerKind::Group(group_open, unsafe { transmute(self) }),
                        next: None,
                    });

                    group_tokenizer.update_next();

                    LazyToken::Group(group_open, group_tokenizer)
                }
            }),
        }
    }

    pub fn peek(&self) -> Option<PeekToken> {
        self.next
    }

    pub fn context(&self) -> ContextHandle<'ctx> {
        self.context
    }

    pub fn open_delimeter(&self) -> Option<OpenDelimiter> {
        match &self.kind {
            TokenizerKind::Group(open, _) => Some(*open),
            TokenizerKind::Root(_) => None,
        }
    }

    fn update_next(&mut self) {
        if self.next.is_none() {
            return;
        }

        self.next = match self.raw().next() {
            None => None,
            Some(raw_token) => Some(match raw_token {
                RawToken::Ident(raw_token) => PeekToken::Ident(raw_token),
                RawToken::Keyword(raw_token) => PeekToken::Keyword(raw_token),
                RawToken::Punct(raw_token) => PeekToken::Punct(raw_token),
                RawToken::Literal(raw_token) => PeekToken::Literal(raw_token),
                RawToken::OpenDelimiter(raw_token) => PeekToken::Group(raw_token),
                RawToken::CloseDelimiter(close) => {
                    self.close(close);

                    return;
                }
            }),
        };
    }

    fn close(&mut self, close: CloseDelimiter) {
        self.next = None;

        match &mut self.kind {
            TokenizerKind::Root(_) => {
                self.context.push_error(TokenError::Unopened(close));
            }
            TokenizerKind::Group(open, parent) => {
                if open.kind != close.kind {
                    parent.close(close);
                } else {
                    parent.update_next();
                }
            }
        }
    }

    fn raw(&mut self) -> &mut RawTokenizer<'src, 'ctx> {
        match &mut self.kind {
            TokenizerKind::Root(raw) => raw,
            TokenizerKind::Group(_, parent) => parent.raw(),
        }
    }
}

impl<'src, 'ctx, 'parent> Drop for Tokenizer<'src, 'ctx, 'parent> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

enum TokenizerKind<'src, 'ctx, 'parent> {
    Root(RawTokenizer<'src, 'ctx>),
    Group(OpenDelimiter, &'parent mut Tokenizer<'src, 'ctx, 'parent>),
}
