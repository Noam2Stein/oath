use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::*;

pub struct Parser<'src, 'ctx, T: Tokenizer<'src, 'ctx>>(pub T, PhantomData<&'src ()>, PhantomData<&'ctx ()>);

impl<'src, 'ctx, T: Tokenizer<'src, 'ctx>> Deref for Parser<'src, 'ctx, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'src, 'ctx, T: Tokenizer<'src, 'ctx>> DerefMut for Parser<'src, 'ctx, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'src, 'ctx, T: Tokenizer<'src, 'ctx>> Drop for Parser<'src, 'ctx, T> {
    fn drop(&mut self) {
        fn into_span(token: LazyToken) -> Span {
            match token {
                LazyToken::Ident(token) => token.span(),
                LazyToken::Keyword(token) => token.span(),
                LazyToken::Punct(token) => token.span(),
                LazyToken::Literal(token) => token.span(),
                LazyToken::Group(mut token) => token.open().span() + token.close().span(),
            }
        }

        let mut span = match self.next() {
            Some(next) => into_span(next),
            None => return,
        };

        while let Some(next) = self.next() {
            span = span + into_span(next)
        }

        self.context().push_error(SyntaxError::UnexpectedTokens(span));
    }
}
