use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "either `{ }` or `;`"]
pub enum BracesOrSemi<T> {
    Braces(T),
    Semi,
}

impl<T: Parse> Parse for BracesOrSemi<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        if let Some(group) = <Option<Group<Braces>>>::parse(parser) {
            Self::Braces(T::parse(&mut group.into_parser(parser.context())))
        } else {
            if let None = <Option<punct!(";")>>::parse(parser) {
                parser.context().push_error(SyntaxError::Expected(
                    parser.peek_span(),
                    "either `{ }` or `;`",
                ));
            }

            Self::Semi
        }
    }
}
impl<T> Detect for BracesOrSemi<T> {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        <Group<Braces>>::detect(parser) || <punct!(";")>::detect(parser)
    }
}
