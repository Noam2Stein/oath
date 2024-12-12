use crate::{diagnostics::*, tokenization::*};

mod tokens;
mod tokens_peek;
mod tokens_peek_ref;
mod vec;

mod groups;
mod seperated;
mod terminated;
mod wrapped;
pub use groups::*;
pub use seperated::*;
pub use terminated::*;
pub use wrapped::*;

pub use oath_proc_macros::{Parse, Peek};

pub trait Parse {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self;
}
pub trait Peek {
    fn peek(input: &mut impl TokenIterator, errors: &mut ErrorsHandle, bound_to_line: bool)
        -> bool;
}
pub trait PeekRef {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self>;
}

impl<T: Parse + Peek> Parse for Option<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if T::peek(input, errors, bound_to_line) {
            Some(Parse::parse(input, errors, bound_to_line))
        } else {
            None
        }
    }
}
impl<T: Parse> Parse for Box<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        Box::new(Parse::parse(input, errors, bound_to_line))
    }
}
impl<T: Peek> Peek for Box<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        T::peek(input, errors, bound_to_line)
    }
}
