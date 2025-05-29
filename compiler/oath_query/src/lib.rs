use std::{fmt::Debug, marker::PhantomData, sync::RwLock};

use oath_src::*;
use oath_tokens::*;

mod expr;
mod item;
mod lib_;
mod namespace;
pub use expr::*;
pub use item::*;
pub use lib_::*;
pub use namespace::*;

#[derive(Debug)]
pub struct QueryContext {
    exprs: QueryBuffer<Expr>,
    libs: QueryBuffer<Lib>,
}

pub trait QueryType: Sized + Debug {
    type Ast: Debug;

    fn buf(context: &QueryContext) -> &QueryBuffer<Self>;

    fn eval(ast: &Self::Ast, namespace: &impl Namespace, context: &QueryContext) -> Self;
}

#[derive(Debug)]
pub struct Owned<T: QueryType>(usize, PhantomData<T>);
#[derive(Debug)]
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
