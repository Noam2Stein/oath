use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a fn"]
pub struct Fn {
    pub vis: Vis,
    pub con: Option<keyword!("con")>,
    pub raw: Option<keyword!("raw")>,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub params: Vec<FnParam>,
    pub output: Option<Try<Expr>>,
    pub contract: Contract,
    pub block: BracesOrSemi<()>,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a fn param"]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Try<Ident>,
    pub type_: Try<Expr>,
    pub bounds: Option<Try<Expr>>,
}

impl ItemParse for Fn {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> Self {
        let vis = modifiers.take_vis();
        let con = modifiers.take_con();
        let raw = modifiers.take_raw();

        target_kind.expect_empty(parser.context(), Self::desc());

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
                    block: BracesOrSemi::Semi,
                };
            }
        };

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
                    block: BracesOrSemi::Semi,
                }
            }
        };

        let output = <Option<punct!("->")>>::parse(parser).map(|_| Parse::parse(parser));

        let contract = Parse::parse(parser);
        let block = Parse::parse(parser);

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

        let type_ = if let Some(_) = <Option<punct!("-")>>::parse(parser) {
            Expr::try_parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`param_ident-ParamType`",
            ));

            Try::Failure
        };

        let bounds = <Option<punct!(":")>>::parse(parser).map(|_| Parse::parse(parser));

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
