use crate::*;

#[derive(Debug, Clone, Parse)]
pub struct Sys {
    #[dont_parse]
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub _semi: Try<punct!(";")>,
}

impl ItemType for Sys {
    fn add_modifiers(
        &mut self,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) {
        item_kind.expect_no_target(context);

        self.vis = modifiers.take_vis();
    }
}
