use crate::*;

#[derive(Debug, Clone, OptionSpanned)]

pub enum VarName {
    Tuple(#[span] Span, Vec<VarName>),
    Ident(
        #[option_spanned] Option<keyword!("mut")>,
        #[option_spanned] Try<Ident>,
        #[option_spanned] Option<Try<Expr>>,
    ),
}

impl OptionParse for VarName {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        if let Some(mut_) = <keyword!("mut")>::option_parse(parser) {
            let ident = Ident::try_parse(parser);

            parser.context().highlight(ident, HighlightColor::Cyan);
            ident.expect_case(IdentCase::LowerCamelCase, parser.context());

            let type_ = <punct!("-")>::option_parse(parser).map(|_| Expr::try_parse_no_mhs(parser));

            return Some(Self::Ident(Some(mut_), ident, type_));
        }

        if let Some(ident) = Ident::option_parse(parser) {
            parser.context().highlight(ident, HighlightColor::Cyan);
            ident.expect_case(IdentCase::LowerCamelCase, parser.context());

            let type_ = <punct!("-")>::option_parse(parser).map(|_| Expr::try_parse_no_mhs(parser));

            return Some(Self::Ident(None, Try::Success(ident), type_));
        }

        if let Some(group) = Group::<Parens>::option_parse(parser) {
            return Some(Self::Tuple(
                group.span(),
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>(),
            ));
        }

        None
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Ident::detect(parser) || Group::<Parens>::detect(parser)
    }

    fn desc() -> &'static str {
        "either an ident or `( )`"
    }
}
