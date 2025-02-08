use std::{fmt::Debug, hash::Hash};

use crate::*;

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
    + TryFrom<TokenTree>
    + for<'a> TryFrom<&'a TokenTree>
{
}
