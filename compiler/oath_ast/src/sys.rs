use crate::*;

#[derive(Debug, Clone, Parse)]
pub struct Sys {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
}

impl ItemParse for Sys {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) -> Self {
        let mut output = Self::parse_error();

        match item_kind.expect_no_target(parser.context()) {
            Try::Success(_) => {}
            Try::Failure => return output,
        }

        output.vis = modifiers.take_vis();

        output.ident = match Ident::try_parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => return output,
        };

        parser
            .context()
            .highlight(output.ident, HighlightColor::Green);
        output
            .ident
            .expect_case(IdentCase::UpperCamelCase, parser.context());

        output.generics = Parse::parse(parser);
        output.contract = Parse::parse(parser);

        match <punct!(";")>::try_parse(parser) {
            Try::Success(_) => {}
            Try::Failure => return output,
        }

        output
    }
}
