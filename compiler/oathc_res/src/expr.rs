use super::*;

#[derive(Debug, Spanned)]
pub enum Expr {
    TypeItem(#[span] Span, Id<TypeItem>),
}

impl ResType for Expr {
    type Src = oath_ast::Expr;

    fn buf(context: &ResContext) -> &QueryBuffer<Self> {
        &context.exprs
    }

    fn eval(ast: &Self::Src, namespace: &impl Namespace, context: &ResContext) -> Self {}
}
