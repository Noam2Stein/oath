use std::marker::PhantomData;

use super::*;

#[derive(Debug, Clone, Hash)]
pub struct LineTerminated<I: Parse> {
    pub items: Vec<I>,
}
impl<I: Parse> Parse for LineTerminated<I> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if !bound_to_line && input.peek(errors, true).is_none() {
            input.next_line(errors);
        }

        let mut items = Vec::new();

        while input.peek(errors, true).is_some() {
            items.push(I::parse(input, errors, true));
            input.next_line(errors);
        }

        Self { items }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct TerminatedTrailing<I: Parse, S: Parse> {
    pub items: Vec<I>,
    pub danny: PhantomData<S>,
}
impl<I: Parse, S: Parse> Parse for TerminatedTrailing<I, S> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut items = Vec::new();

        while input.peek(errors, bound_to_line).is_some() {
            items.push(I::parse(input, errors, bound_to_line));
            S::parse(input, errors, bound_to_line);
        }

        Self {
            items,
            danny: PhantomData::default(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct TerminatedNotTrailing<I: Parse, S: Parse> {
    pub items: Vec<I>,
    pub danny: PhantomData<S>,
}
impl<I: Parse, S: Parse> Parse for TerminatedNotTrailing<I, S> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut items = Vec::new();

        if input.peek(errors, bound_to_line).is_some() {
            items.push(I::parse(input, errors, bound_to_line));

            while input.peek(errors, bound_to_line).is_some() {
                S::parse(input, errors, bound_to_line);
                items.push(I::parse(input, errors, bound_to_line));
            }
        }

        Self {
            items,
            danny: PhantomData::default(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct TerminatedMaybeTrailing<I: Parse, S: Parse> {
    pub items: Vec<I>,
    pub danny: PhantomData<S>,
}
impl<I: Parse, S: Parse> Parse for TerminatedMaybeTrailing<I, S> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut items = Vec::new();

        while input.peek(errors, bound_to_line).is_some() {
            items.push(I::parse(input, errors, bound_to_line));

            if input.peek(errors, bound_to_line).is_some() {
                S::parse(input, errors, bound_to_line);
            }
        }

        Self {
            items,
            danny: PhantomData::default(),
        }
    }
}
