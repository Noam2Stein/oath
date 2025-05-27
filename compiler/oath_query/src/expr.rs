use super::*;

#[derive(Debug)]
pub enum Expr {}

impl QueryType for Expr {
    fn buf(context: &QueryContext) -> &QueryBuffer<Self> {
        &context.exprs
    }
}
