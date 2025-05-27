use std::{fmt::Debug, marker::PhantomData, sync::RwLock};

mod expr;
mod item;
mod lib_;
mod mod_;
pub use expr::*;
pub use item::*;
pub use lib_::*;
pub use mod_::*;

#[derive(Debug)]
pub struct QueryContext {
    exprs: QueryBuffer<Expr>,
}

pub trait QueryType: Sized + Debug {
    type Ast: Debug;

    fn buf(context: &QueryContext) -> &QueryBuffer<Self>;
}

#[derive(Debug)]
pub struct Owned<T: QueryType>(usize, PhantomData<T>);
pub struct Dep<T: QueryType>(usize, PhantomData<T>);

#[derive(Debug)]
struct QueryBuffer<T: QueryType> {
    nodes: RwLock<Vec<Query<T>>>,
}

#[derive(Debug)]
struct Query<T: QueryType> {
    ast: T::Ast,
    value: Option<T>,
    dependent_exprs: Vec<usize>,
}
