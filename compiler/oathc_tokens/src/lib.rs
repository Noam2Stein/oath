use std::{fmt::Debug, hash::Hash};

use derive_more::*;
use derive_new::*;
use oathc_diagnostics::*;
use oathc_interner::*;
use oathc_span::*;

pub use oathc_token_definitions::*;
pub use oathc_tokens_proc_macros::{close, delims, keyword, open, punct};

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

#[allow(dead_code)]
const fn verify_token_type<T: Debug + Spanned>() {}
