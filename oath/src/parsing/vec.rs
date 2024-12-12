use super::*;

impl<T: Parse + Peek> Parse for Vec<T> {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        let mut output = Vec::new();
        while let Some(item) = Option::<T>::parse(input, errors, bound_to_line) {
            output.push(item);
        }

        output
    }
}
impl<T: Peek> Peek for Vec<T> {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        T::peek(input, errors, bound_to_line)
    }
}
