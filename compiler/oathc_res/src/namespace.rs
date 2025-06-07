use super::*;

pub trait Namespace {
    fn get(&self, ident: Ident, oath: &ResContext) -> Expr;
}

pub enum Name {
    TypeItem(Id<TypeItem>),
}
