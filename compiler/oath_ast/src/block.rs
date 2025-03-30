use crate::*;

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "a block"]
pub struct Block {
    #[span]
    span: Span,
    pub stmts: Vec<Stmt>,
    pub has_trailing_semi: bool,
}

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "a stmt"]
pub enum Stmt {
    Var(VarStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "a variable declaration"]
pub struct VarStmt {
    #[span]
    span: Span,
    pub name: Try<VarName>,
    pub init: Try<Expr>,
}

impl OptionParse for Block {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        Group::<Braces>::option_parse(parser).map(|group| {
            let span = group.span();
            let mut parser = group.into_parser(parser.context());

            let mut stmts = Vec::new();
            let mut has_trailing_semi = true;

            while let Some(value) = Stmt::option_parse(&mut parser) {
                stmts.push(value);

                if <punct!(";")>::option_parse(&mut parser).is_none() {
                    has_trailing_semi = false;
                    break;
                }
            }

            match stmts.last() {
                Some(Stmt::Expr(_)) | None => {}
                _ => {
                    if !has_trailing_semi {
                        parser.context().push_error(SyntaxError::Expected(
                            parser.peek_span(),
                            <punct!(";")>::desc(),
                        ));
                    }
                }
            }

            Self {
                span,
                stmts,
                has_trailing_semi,
            }
        })
    }
}
impl Detect for Block {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Braces>::detect(parser)
    }
}

impl OptionParse for VarStmt {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let var_keyword = <keyword!("var")>::option_parse(parser)?;

        let name = VarName::try_parse(parser);
        if name.is_failure() {
            let span = parser.peek_span();

            parser.skip_until(|parser| {
                <punct!(";")>::detect(parser) || <punct!("=")>::detect(parser)
            });

            if !<punct!("=")>::detect(parser) {
                return Some(Self {
                    name: Try::Failure,
                    init: Try::Failure,
                    span,
                });
            }
        }

        let mut span = var_keyword.span() + parser.peek_span();

        let init = <punct!("=")>::option_parse(parser).map(|_| Expr::try_parse(parser));
        if let Some(Try::Success(init)) = &init {
            span += init.span();
        }

        let init = if let Some(init) = init {
            init
        } else {
            parser.context().push_error(Error::new(
                "uninit variables are not allowed",
                parser.peek_span(),
            ));

            Try::Failure
        };

        Some(Self { name, init, span })
    }
}
impl Detect for VarStmt {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        <keyword!("var")>::detect(parser)
    }
}
