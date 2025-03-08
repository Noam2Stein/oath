use oath_ast::Fields;

#[derive(Debug, Clone)]
pub struct Type {
    pub generics: oath_ast::GenericParams,
    pub contract: oath_ast::Contract,
    pub content: TypeContent,
}

#[derive(Debug, Clone)]
pub enum TypeContent {
    Fields(Fields),
    None,
}
