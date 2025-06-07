use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::take,
    sync::{RwLock, RwLockWriteGuard},
};

use oathc_span::*;
use oathc_tokens::*;

mod expr;
mod item;
mod namespace;
mod res_type;
pub use expr::*;
pub use item::*;
pub use namespace::*;
pub use res_type::*;

#[derive(Debug)]
pub struct ResContext {
    exprs: QueryBuffer<Expr>,
    mods: QueryBuffer<Mod>,
}

pub trait ResType: Sized + Debug {
    type Src: Debug;

    fn buf(context: &ResContext) -> &QueryBuffer<Self>;

    fn drop_src(src: Self::Src, context: &ResContext);

    fn eval(ast: &Self::Src, namespace: &impl Namespace, context: &ResContext) -> Self;
}

#[derive(Debug)]
pub struct Id<T: ResType>(usize, PhantomData<T>);

pub struct ResChanges<'ctx> {
    ctx: &'ctx ResContext,
    dirty_lock: RwLockWriteGuard<'ctx, Vec<Id<Expr>>>,
}

impl ResContext {
    pub fn update(&self, report_changes: impl FnOnce(ResChanges)) {
        report_changes(ResChanges { ctx: self });
    }

    pub fn replace<T: ResType>(&self, id: &Id<T>, src: T::Src) {
        self.make_dirty(id);

        let mut resolvables = T::buf(self).resolvables.write().unwrap();
        let res = resolvables[id.0].as_mut().unwrap();

        res.src = src;
    }

    fn make_dirty<T: ResType>(&self, id: &Id<T>) {
        let mut dependents = {
            let mut resolvables = T::buf(self).resolvables.write().unwrap();
            let res = resolvables[id.0].as_mut().unwrap();

            res.value = None;

            take(&mut res.dependents)
        };

        let mut resolvables = Expr::buf(self).resolvables.write().unwrap();

        while let Some(dependent) = dependents.pop() {
            let res = resolvables[dependent.0].as_mut().unwrap();

            res.value = None;
            dependents.append(&mut res.dependents);
        }
    }
}
impl<'ctx> ResChanges<'ctx> {}

#[derive(Debug)]
struct QueryBuffer<T: ResType> {
    resolvables: RwLock<Vec<Option<Res<T>>>>,
    dirty: RwLock<Vec<Id<T>>>,
}

#[derive(Debug)]
struct Res<T: ResType> {
    src: T::Src,
    value: Option<T>,
    dependents: Vec<Id<Expr>>,
}
