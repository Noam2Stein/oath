use std::{fmt::Debug, hash::Hash};

use derive_more::*;
use derive_new::*;

use super::interner::*;
use super::span::*;

// Pub

pub use oathc_token_definitions::*;

pub const KEYWORDS: &[&str] = with_tokens_expr! {
    &[$($keyword), *]
};
pub fn is_keyword(str: &str) -> bool {
    with_tokens_expr! {
        match str {
            $($keyword => true,)*
            _ => false,
        }
    }
}

pub const PUNCTS: &[&str] = with_tokens_expr! {
    &[$($punct), *]
};
pub fn is_punct(str: &str) -> bool {
    with_tokens_expr! {
        match str {
            $($punct => true,)*
            _ => false,
        }
    }
}

// Pub(Super)

pub(super) use oathc_proc_macros::{close, delims, keyword, open, punct};

mod char_literal;
mod delimiters;
mod float_literal;
mod ident;
mod int_literal;
mod keyword;
mod literal;
mod punct;
mod str_literal;
pub(super) use char_literal::*;
pub(super) use delimiters::*;
pub(super) use float_literal::*;
pub(super) use ident::*;
pub(super) use int_literal::*;
pub(super) use keyword::*;
pub(super) use literal::*;
pub(super) use punct::*;
pub(super) use str_literal::*;

// Private

#[allow(dead_code)]
const fn verify_token_type<T: Debug + Copy + Eq + Ord + Hash + InternedDisplay + Spanned>() {}
