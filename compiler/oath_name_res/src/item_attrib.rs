#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemAttrib {
    Pub(oath_ast::Pub),
    Con(oath_tokenizer::ConKeyword),
    Raw(oath_tokenizer::RawKeyword),
}
