use crate::*;

#[derive(Debug, Clone)]
pub struct Func {
    pub vis: Vis,
    pub con: Option<keyword!("con")>,
    pub raw: Option<keyword!("raw")>,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub params: Vec<FnParam>,
    pub output_try: Option<OutputTry>,
    pub output: Option<Try<Expr>>,
    pub contract: Contract,
    pub block: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Try<Ident>,
    pub type_: Try<Expr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "output try"]
pub struct OutputTry {
    pub keyword: keyword!("try"),
    pub generics: Option<GenericArgs>,
}

impl ItemParse for Func {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) -> Self {
        let vis = modifiers.take_vis();
        let con = modifiers.take_con();
        let raw = modifiers.take_raw();

        let _ = item_kind.expect_no_target(parser.context());

        let ident = Ident::try_parse(parser);
        if ident.is_failure() {
            parser.skip_until(|parser| <punct!(",")>::detect(parser));

            return Self {
                vis,
                con,
                raw,
                ident: Try::Failure,
                generics: None,
                params: Vec::new(),
                output_try: None,
                output: None,
                contract: Default::default(),
                block: None,
            };
        }

        parser.context().highlight(ident, HighlightColor::Yellow);
        ident.expect_case(IdentCase::LowerCamelCase, parser.context());

        let generics = Parse::parse(parser);

        let params = match Group::<Parens>::try_parse(parser) {
            Try::Success(group) => group
                .into_parser(parser.context())
                .parse_trl::<_, punct!(",")>(),
            Try::Failure => {
                return Self {
                    vis,
                    con,
                    raw,
                    ident,
                    generics,
                    params: Vec::new(),
                    output_try: None,
                    output: None,
                    contract: Default::default(),
                    block: None,
                }
            }
        };

        let output_arrow = <punct!("->")>::option_parse(parser);
        let output_try = output_arrow.map_or(None, |_| OutputTry::option_parse(parser));
        let output = output_arrow.map(|_| Expr::try_parse(parser));

        let contract = Parse::parse(parser);
        let block = Block::option_parse(parser);

        if block.is_none() && <punct!(";")>::option_parse(parser).is_none() {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "either `{}` or `;`",
            ));
        }

        Self {
            raw,
            vis,
            con,
            contract,
            generics,
            ident,
            params,
            output_try,
            output,
            block,
        }
    }
}

impl Parse for FnParam {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let mut_ = Parse::parse(parser);

        let ident = match Parse::parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                parser.skip_until(|parser| <punct!(",")>::detect(parser));

                return Self {
                    mut_,
                    ident: Try::Failure,
                    type_: Try::Failure,
                    bounds: None,
                };
            }
        };

        parser.context().highlight(ident, HighlightColor::Cyan);
        ident.expect_case(IdentCase::LowerCamelCase, parser.context());

        let type_ = if let Some(_) = <Option<punct!("-")>>::parse(parser) {
            Expr::try_parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`param_ident-ParamType`",
            ));

            Try::Failure
        };

        let bounds = Bounds::option_parse(parser);

        Self {
            mut_,
            ident,
            type_,
            bounds,
        }
    }

    fn parse_error() -> Self {
        todo!()
    }
}
