use std::ops::{Deref, DerefMut};

use crate::*;

pub struct Parser<'src, 'ctx, 'parent>(pub Tokenizer<'src, 'ctx, 'parent>);

impl<'src, 'ctx, 'parent> Deref for Parser<'src, 'ctx, 'parent> {
    type Target = Tokenizer<'src, 'ctx, 'parent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'src, 'ctx, 'parent> DerefMut for Parser<'src, 'ctx, 'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'src, 'ctx, 'parent> Drop for Parser<'src, 'ctx, 'parent> {
    fn drop(&mut self) {
        let mut span = match self.next() {
            Some(next) => next.span(),
            None => return,
        };

        while let Some(next) = self.next() {
            span = span + next.span()
        }

        self.context().push_error(SyntaxError::UnexpectedTokens(span));
    }
}
