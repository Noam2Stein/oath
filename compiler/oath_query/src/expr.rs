use super::*;

#[derive(Debug, Spanned)]
pub enum Expr {
    TypeItem(#[span] Span, Dep<TypeItem>),
}

impl QueryType for Expr {
    type Ast = oath_ast::Expr;

    fn buf(context: &QueryContext) -> &QueryBuffer<Self> {
        &context.exprs
    }

    fn eval(ast: &Self::Ast, namespace: &impl Namespace, context: &QueryContext) -> Self {}
}
