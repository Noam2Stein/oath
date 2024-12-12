use super::*;

#[derive(Debug, Clone, Hash, Parse)]
pub struct Item<C: std::fmt::Debug + Clone + std::hash::Hash + Parse = ItemContent> {
    pub vis: Vis,
    pub content: C,
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub enum ItemContent {
    Mod(Mod),
    Func(Func),
    Struct(Struct),
    #[error]
    Error(Span),
}

#[derive(Debug, Clone, Hash, Parse)]
pub struct Vis {
    pub r#pub: Option<Keyword!("pub")>,
}
