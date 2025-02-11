use crate::*;

impl<T: Parse> Parse for Box<T> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        parser.parse(context).map(|ok| Box::new(ok))
    }
}

impl<T: Peek> Peek for Box<T> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        T::peek(parser, context)
    }
}
