use crate::*;

#[derive(Debug, Clone, OptionSpanned)]
pub struct Bounds {
    #[option_spanned]
    pub expr: Try<Expr>,
}

impl OptionParse for Bounds {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        if let Some(_) = <punct!(":")>::option_parse(parser) {
            let expr = Expr::try_parse(parser);

            return Some(Self { expr });
        }

        if let Some(punct) = <punct!("=")>::option_parse(parser) {
            let eq_expr = Expr::try_parse(parser);

            return Some(Self {
                expr: Try::Success(Expr::ShsOp(
                    punct.span() + eq_expr.option_span(),
                    ShsOp::Eq(EqEqPunct(punct.span())),
                    eq_expr.map_box(),
                )),
            });
        }

        None
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        <punct!(":")>::detect(parser) || <punct!("=")>::detect(parser)
    }

    fn desc() -> &'static str {
        "either `:` or `=`"
    }
}
