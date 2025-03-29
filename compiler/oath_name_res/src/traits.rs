use std::fmt::Debug;

use oath_context::ContextHandle;

use crate::*;

pub trait IntoNamespace {
    fn into_namespace(
        self,
        name_context: &mut DumbNameContext,
        context: ContextHandle,
    ) -> NamespaceId;
}

pub trait NameRes {
    type Enviorment;
    type Output: Debug + Clone;

    fn name_res(
        self,
        env: Self::Enviorment,
        name_context: &mut DumbNameContext,
        context: ContextHandle,
    ) -> Self::Output;
}
