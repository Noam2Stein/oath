use crate::*;

#[derive(Debug, Clone, Parse)]
#[desc = "a visibility"]
pub enum Vis {
    Pub(keyword!("pub")),
    #[fallback]
    Priv,
}
