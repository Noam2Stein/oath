use std::{fmt::Debug, hash::Hash};

use derive_more::*;
use derive_new::*;

use super::interner::*;
use super::span::*;

pub use oathc_proc_macros::{close, delims, keyword, open, punct};
pub use oathc_token_definitions::*;

mod char_literal;
mod delimiters;
mod float_literal;
mod ident;
mod int_literal;
mod keyword;
mod literal;
mod punct;
mod str_literal;
pub use char_literal::*;
pub use delimiters::*;
pub use float_literal::*;
pub use ident::*;
pub use int_literal::*;
pub use keyword::*;
pub use literal::*;
pub use punct::*;
pub use str_literal::*;

const fn verify_token_type<T: Debug + Copy + Eq + Ord + Hash + InternedDisplay + Spanned>() {}
