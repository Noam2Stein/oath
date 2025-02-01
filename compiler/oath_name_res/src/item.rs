use crate::Mod;

#[derive(Debug, Clone)]
pub enum Item {
    Mod(Mod),
    Struct(oath_ast::Struct),
}
