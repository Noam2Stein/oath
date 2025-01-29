use oath_diagnostics::{Desc, Fill};

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
pub trait TokenType: Seal + Desc + Fill + TokenDowncastFrom<TokenTree> {}

pub trait TokenDowncastFrom<F>: Sized {
    fn downcast_from(value: F) -> Option<Self>;
    fn downcast_from_ref(value: &F) -> Option<&Self>;
}
pub trait TokenDowncast<I: TokenDowncastFrom<Self>>: Sized {
    fn downcast(self) -> Option<I> {
        I::downcast_from(self)
    }
    fn downcast_ref(&self) -> Option<&I> {
        I::downcast_from_ref(self)
    }
}
impl<T, I: TokenDowncastFrom<T>> TokenDowncast<I> for T {}
