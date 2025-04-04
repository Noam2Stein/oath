use crate::*;

#[derive(Debug, Clone, Parse)]
pub enum Vis {
    Pub(keyword!("pub")),
    #[fallback]
    Priv,
}
