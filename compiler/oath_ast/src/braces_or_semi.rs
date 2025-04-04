use crate::*;

#[derive(Debug, Clone)]
pub enum BracesOrSemi<T> {
    Braces(T),
    Semi,
}

impl<T: Parse> OptionParse for BracesOrSemi<T> {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        if let Some(group) = <Option<Group<Braces>>>::parse(parser) {
            Some(Self::Braces(T::parse(
                &mut group.into_parser(parser.context()),
            )))
        } else if <punct!(";")>::option_parse(parser).is_some() {
            Some(Self::Semi)
        } else {
            None
        }
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Braces>::detect(parser) || <punct!(";")>::detect(parser)
    }

    fn desc() -> &'static str {
        "either `{ }` or `;`"
    }
}
