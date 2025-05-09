use std::ops::{Deref, DerefMut};

use crate::*;

pub struct Parser<Src: TokenSource>(pub Tokenizer<Src>);

impl<Src: TokenSource> Deref for Parser<Src> {
    type Target = Tokenizer<Src>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<Src: TokenSource> DerefMut for Parser<Src> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Src: TokenSource> Drop for Parser<Src> {
    fn drop(&mut self) {
        fn into_span(token: LazyToken<impl TokenSource>) -> Span {
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
