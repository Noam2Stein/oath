use derivative::Derivative;

use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct InDelimiters<T: Parse> {
    pub delimiters: Delimiters,
    pub inner: T,
}
#[derive(Derivative)]
#[derivative(Debug, Clone, Copy, Hash)]
pub struct InAngles<T: Parse> {
    pub open: Punct!("<"),
    pub inner: T,
    pub close: Punct!(">"),
}
#[derive(Debug, Clone, Copy, Hash)]
pub struct InBraces<T: Parse> {
    pub delimiters: Braces,
    pub inner: T,
}
#[derive(Debug, Clone, Copy, Hash)]
pub struct InBrackets<T: Parse> {
    pub delimiters: Brackets,
    pub inner: T,
}
#[derive(Debug, Clone, Copy, Hash)]
pub struct InParens<T: Parse> {
    pub delimiters: Parens,
    pub inner: T,
}

impl<T: Parse> Parse for InDelimiters<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let group = Group::parse(input, errors, bound_to_line);
        let input = &mut group.tokens.into_token_iter(group.delimiters.close_span());

        let delimiters = group.delimiters;
        let inner = T::parse(input, errors, false);

        if input.peek(errors, bound_to_line).is_some() {
            let span = input.peek_span(errors, bound_to_line);

            errors.push(Error::new(span, "unexpected tokens"));
        }

        Self { delimiters, inner }
    }
}
impl<T: Parse> Parse for InAngles<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        Self {
            open: Parse::parse(input, errors, bound_to_line),
            inner: Parse::parse(input, errors, bound_to_line),
            close: Parse::parse(input, errors, bound_to_line),
        }
    }
}
impl<T: Parse> Parse for InBraces<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let group = GroupBraces::parse(input, errors, bound_to_line);
        let input = &mut group.tokens.into_token_iter(group.delimiters.close_span());

        let delimiters = group.delimiters;
        let inner = T::parse(input, errors, false);

        if input.peek(errors, bound_to_line).is_some() {
            let span = input.peek_span(errors, bound_to_line);

            errors.push(Error::new(span, "unexpected tokens"));
        }

        Self { delimiters, inner }
    }
}
impl<T: Parse> Parse for InBrackets<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let group = GroupBrackets::parse(input, errors, bound_to_line);
        let input = &mut group.tokens.into_token_iter(group.delimiters.close_span());

        let delimiters = group.delimiters;
        let inner = T::parse(input, errors, false);

        if input.peek(errors, bound_to_line).is_some() {
            let span = input.peek_span(errors, bound_to_line);

            errors.push(Error::new(span, "unexpected tokens"));
        }

        Self { delimiters, inner }
    }
}
impl<T: Parse> Parse for InParens<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let group = GroupParens::parse(input, errors, bound_to_line);
        let input = &mut group.tokens.into_token_iter(group.delimiters.close_span());

        let delimiters = group.delimiters;
        let inner = T::parse(input, errors, false);

        if input.peek(errors, bound_to_line).is_some() {
            let span = input.peek_span(errors, bound_to_line);

            errors.push(Error::new(span, "unexpected tokens"));
        }

        Self { delimiters, inner }
    }
}

impl<T: Parse> Peek for InDelimiters<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Group::peek(input, errors, bound_to_line)
    }
}
impl<T: Parse> Peek for InAngles<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        <Punct!("<")>::peek(input, errors, bound_to_line)
    }
}
impl<T: Parse> Peek for InBraces<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Braces::peek(input, errors, bound_to_line)
    }
}
impl<T: Parse> Peek for InBrackets<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Brackets::peek(input, errors, bound_to_line)
    }
}
impl<T: Parse> Peek for InParens<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Parens::peek(input, errors, bound_to_line)
    }
}
