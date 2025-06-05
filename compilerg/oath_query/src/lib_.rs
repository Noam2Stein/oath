use super::*;

#[derive(Debug)]
pub struct Lib {
    types: Vec<Owned<TypeItem>>,
}

impl QueryType for Lib {
    type Ast = oath_ast::SyntaxTree;

    fn buf(context: &QueryContext) -> &QueryBuffer<Self> {
        &context.libs
    }

    
}
