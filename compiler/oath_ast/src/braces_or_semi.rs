use crate::*;

#[derive(Debug, Clone, Default, Desc)]
#[desc = "either `{ }` or `;`"]
pub struct BracesOrSemi<T> {
    pub inner: Option<T>,
}

impl<T: Parse> Parse for BracesOrSemi<T> {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        Self {
            inner: if let Some(group) = parser.parse::<Option<Group<Braces>>>(context) {
                Some(group.into_parser().parse_all(context))
            } else {
                if let None = parser.parse::<Option<punct!(";")>>(context) {
                    context.push_error(SyntaxError::Expected(
                        parser.next_span(),
                        "either `{ }` or `;`",
                    ));
                }
                None
            },
        }
    }
}

impl<T: TryParse> TryParse for BracesOrSemi<T> {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        Ok(Self {
            inner: if let Some(group) = parser.parse::<Option<Group<Braces>>>(context) {
                Some(group.into_parser().try_parse_all(context)?)
            } else {
                if let None = parser.parse::<Option<punct!(";")>>(context) {
                    context.push_error(SyntaxError::Expected(
                        parser.next_span(),
                        "either `{ }` or `;`",
                    ));
                }
                None
            },
        })
    }
}
