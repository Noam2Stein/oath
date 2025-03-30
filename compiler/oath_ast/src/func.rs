use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a fn"]
pub struct Func {
    pub vis: Vis,
    pub con: Option<keyword!("con")>,
    pub raw: Option<keyword!("raw")>,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub params: Vec<FnParam>,
    pub output: Option<Try<Expr>>,
    pub contract: Contract,
    pub block: Option<Block>,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a fn param"]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Try<Ident>,
    pub type_: Try<Expr>,
    pub bounds: Option<Bounds>,
}

impl ItemParse for Func {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        target_kind: Option<ItemKind>,
        _kind_keyword: ItemKeyword,
    ) -> Self {
        let vis = modifiers.take_vis();
        let con = modifiers.take_con();
        let raw = modifiers.take_raw();

        if let Some(target_kind) = target_kind {
            parser.context().push_error(SyntaxError::CannotHaveTarget(
                target_kind.span(),
                Self::desc(),
            ));
        };

        let ident = match Parse::parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                parser.skip_until(|parser| <punct!(",")>::detect(parser));

                return Self {
                    vis,
                    con,
                    raw,
                    ident: Try::Failure,
                    generics: None,
                    params: Vec::new(),
                    output: None,
                    contract: Default::default(),
                    block: None,
                };
            }
        };

        parser.context().highlight(ident, HighlightColor::Yellow);
        ident.expect_case(IdentCase::LowerCamelCase, parser.context());

        let generics = Parse::parse(parser);

        let params = match <Try<Group<Parens>>>::parse(parser) {
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
                    output: None,
                    contract: Default::default(),
                    block: None,
                }
            }
        };

        let output = <Option<punct!("->")>>::parse(parser).map(|_| Parse::parse(parser));

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
}

impl Detect for FnParam {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Ident::detect(parser) || <keyword!("mut")>::detect(parser)
    }
}
