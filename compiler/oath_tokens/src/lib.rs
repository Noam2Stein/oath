use std::{fmt::Debug, hash::Hash};

use derive_more::*;
use derive_new::*;

use oath_interner::*;
use oath_src::*;

pub use oath_token_definitions::{with_tokens, with_tokens_expr};
pub use oath_tokens_proc_macros::{keyword, punct};

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

#[macro_export(local_inner_macros)]
macro_rules! verify_token_type {
    ($type:ty) => {
        const _: () = verify_token_type_helper::<$type>();
    };
}

#[allow(dead_code)]
const fn verify_token_type_helper<
    T: Debug
        + Clone
        + Eq
        + Ord
        + Hash
        + TryFrom<TokenTree>
        + for<'a> TryFrom<&'a TokenTree>
        + Into<TokenTree>
        + InternedDisplay
        + Spanned,
>() {
}
