use crate::{IntoNamespace, NameId, NameRes};

#[derive(Debug, Clone)]
pub struct ModContent {
    pub items: Vec<NameId>,
}

impl IntoNamespace for oath_ast::ModContent {
    fn into_namespace(
        self,
        name_context: &mut crate::DumbNameContext,
        context: oath_context::ContextHandle,
    ) -> crate::NamespaceId {
        todo!()
    }
}

impl NameRes for oath_ast::ModContent {
    type Enviorment = ();
    type Output = ModContent;

    fn name_res(
        self,
        env: Self::Enviorment,
        name_context: &mut crate::DumbNameContext,
        context: oath_context::ContextHandle,
    ) -> Self::Output {
        todo!()
    }
}
