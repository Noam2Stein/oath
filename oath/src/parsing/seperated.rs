use std::marker::PhantomData;

use super::*;

#[derive(Debug, Clone, Hash)]
pub struct LineSeperated<I: Parse + Peek> {
    pub items: Vec<I>,
}
impl<I: Parse + Peek> Parse for LineSeperated<I> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if !bound_to_line && input.peek(errors, true).is_none() {
            input.next_line(errors);
        }

        let mut items = Vec::new();

        while I::peek(input, errors, true) {
            items.push(I::parse(input, errors, true));
            input.next_line(errors);
        }

        Self { items }
    }
}
impl<I: Parse + Peek> Peek for LineSeperated<I> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        I::peek(input, errors, bound_to_line)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SeperatedTrailing<I: Parse + Peek, S: Parse> {
    pub items: Vec<I>,
    pub danny: PhantomData<S>,
}
impl<I: Parse + Peek, S: Parse> Parse for SeperatedTrailing<I, S> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut items = Vec::new();

        while I::peek(input, errors, bound_to_line) {
            items.push(I::parse(input, errors, bound_to_line));
            S::parse(input, errors, bound_to_line);
        }

        Self {
            items,
            danny: PhantomData::default(),
        }
    }
}
impl<I: Parse + Peek, S: Parse> Peek for SeperatedTrailing<I, S> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        I::peek(input, errors, bound_to_line)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SeperatedNotTrailing<I: Parse, S: Parse + Peek> {
    pub items: Vec<I>,
    pub danny: PhantomData<S>,
}
impl<I: Parse, S: Parse + Peek> Parse for SeperatedNotTrailing<I, S> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut items = Vec::new();

        items.push(I::parse(input, errors, bound_to_line));

        while S::peek(input, errors, bound_to_line) {
            S::parse(input, errors, bound_to_line);
            items.push(I::parse(input, errors, bound_to_line));
        }

        Self {
            items,
            danny: PhantomData::default(),
        }
    }
}
impl<I: Parse + Peek, S: Parse + Peek> Peek for SeperatedNotTrailing<I, S> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        I::peek(input, errors, bound_to_line)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SeperatedMaybeTrailing<I: Parse + Peek, S: Parse + Peek> {
    pub items: Vec<I>,
    pub danny: PhantomData<S>,
}
impl<I: Parse + Peek, S: Parse + Peek> Parse for SeperatedMaybeTrailing<I, S> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut items = Vec::new();

        while I::peek(input, errors, bound_to_line) {
            items.push(I::parse(input, errors, bound_to_line));

            if !S::peek(input, errors, bound_to_line) {
                break;
            }
            S::parse(input, errors, bound_to_line);
        }

        Self {
            items,
            danny: PhantomData::default(),
        }
    }
}
impl<I: Parse + Peek, S: Parse + Peek> Peek for SeperatedMaybeTrailing<I, S> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        I::peek(input, errors, bound_to_line)
    }
}
