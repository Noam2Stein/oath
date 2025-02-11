use crate::*;

pub enum Type {
    Path(Path),
    Tuple(Vec<Type>),
}

impl Parse for Type {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(value) = parser.parse(context)? {
            Ok(Self::Path(value))
        } else if let Some(group) = parser.parse::<Option<Group<Parens>>>(context).unwrap() {
            Ok(Self::Tuple(
                group
                    .into_parser()
                    .parse_sep_all::<_, punct!(","), false, true>(context)?,
            ))
        } else {
            context.push_error(SyntaxError::Expected(parser.next_span(), "a type"));
            Err(())
        }
    }
}
impl Peek for Type {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Path>(context) || parser.peek::<Group<Parens>>(context)
    }
}
