use crate::*;

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "a block"]
pub struct Block {
    #[span]
    span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Var(VarStmt),
}

#[derive(Debug, Clone, Spanned)]
pub struct VarStmt {
    pub ident: Ident,
    pub ty: Option<Expr>,
    pub init: Option<Expr>,
    #[span]
    span: Span,
}

impl OptionParse for Block {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        Group::<Braces>::option_parse(parser).map(|group| Self { span: group.span() })
    }
}

impl Detect for Block {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Braces>::detect(parser)
    }
}
