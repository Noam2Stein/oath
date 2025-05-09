use std::ops::{Deref, DerefMut};

use crate::*;

pub struct Parser<T: Tokenizer>(pub T);

impl<T: Tokenizer> Deref for Parser<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Tokenizer> DerefMut for Parser<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Tokenizer> Drop for Parser<T> {
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
