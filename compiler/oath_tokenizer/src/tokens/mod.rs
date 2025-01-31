use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_src::Spanned;

use crate::Seal;

mod char_literal;
mod delimiters;
mod file;
mod float_literal;
mod group;
mod ident;
mod int_literal;
mod keyword;
mod literal;
mod punct;
mod str_literal;
mod token_tree;
pub use char_literal::*;
pub use delimiters::*;
pub use file::*;
pub use float_literal::*;
pub use group::*;
pub use ident::*;
pub use int_literal::*;
pub use keyword::*;
pub use literal::*;
pub use punct::*;
pub use str_literal::*;
pub use token_tree::*;

#[allow(private_bounds)]
pub trait TokenType:
    Send
    + Sync
    + Debug
    + Clone
    + Eq
    + Ord
    + Hash
    + Spanned
    + Seal
    + Desc
    + Fill
    + TokenDowncastFrom<TokenTree>
{
}

pub trait TokenDowncastFrom<F>: Sized {
    fn downcast_from(value: F) -> Option<Self>;
    fn downcast_from_ref(value: &F) -> Option<&Self>;
}
impl<T> TokenDowncastFrom<T> for T {
    #[inline(always)]
    fn downcast_from(value: T) -> Option<Self> {
        Some(value)
    }
    #[inline(always)]
    fn downcast_from_ref(value: &T) -> Option<&Self> {
        Some(value)
    }
}
pub trait TokenDowncast: Sized {
    #[inline(always)]
    fn downcast<I: TokenDowncastFrom<Self>>(self) -> Option<I> {
        I::downcast_from(self)
    }
    #[inline(always)]
    fn downcast_ref<I: TokenDowncastFrom<Self>>(&self) -> Option<&I> {
        I::downcast_from_ref(self)
    }
}
impl<T> TokenDowncast for T {}
