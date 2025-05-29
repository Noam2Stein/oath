use super::*;

pub trait Namespace {
    fn get(&self, ident: Ident, oath: &QueryContext) -> Expr;
}
