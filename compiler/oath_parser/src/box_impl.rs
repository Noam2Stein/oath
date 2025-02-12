use crate::*;

impl<T: Desc> Desc for Box<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        Box::new(parser.parse(context))
    }
}

impl<T: TryParse> TryParse for Box<T> {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        parser.try_parse(context).map(|value| Box::new(value))
    }
}

impl<T: Peek> Peek for Box<T> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        T::peek(parser, context)
    }
}
