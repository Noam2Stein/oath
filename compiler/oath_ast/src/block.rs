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
    pub mut_: Option<keyword!("mut")>,
    pub ident: Try<Ident>,
    pub ty: Option<Try<Expr>>,
    pub init: Option<Try<Expr>>,
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
        let mut_ = <keyword!("mut")>::option_parse(parser);

        let var_keyword = if let Some(mut_) = mut_ {
            if let Try::Success(success) = <keyword!("var")>::try_parse(parser) {
                success
            } else {
                return Some(Self {
                    mut_: Some(mut_),
                    ident: Try::Failure,
                    ty: None,
                    init: None,
                    span: mut_.span(),
                });
            }
        } else {
            <keyword!("var")>::option_parse(parser)?
        };

        let ident = match Ident::try_parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                return Some(Self {
                    mut_,
                    ident: Try::Failure,
                    ty: None,
                    init: None,
                    span: parser.peek_span(),
                })
            }
        };

        parser.context().highlight(ident, HighlightColor::Cyan);

        let mut span = var_keyword.span() + parser.peek_span();

        let ty = <punct!("-")>::option_parse(parser).map(|_| Expr::try_parse_no_mhs(parser));
        if let Some(Try::Success(ty)) = &ty {
            span += ty.span();
        }

        let init = <punct!("=")>::option_parse(parser).map(|_| Expr::try_parse(parser));
        if let Some(Try::Success(init)) = &init {
            span += init.span();
        }

        Some(Self {
            mut_,
            span,
            ident,
            ty,
            init,
        })
    }
}
impl Detect for VarStmt {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        <keyword!("var")>::detect(parser) || <keyword!("mut")>::detect(parser)
    }
}
