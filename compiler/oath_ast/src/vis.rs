use crate::*;

#[derive(Debug, Clone, ParseDesc, Parse)]
#[desc = "a visibility"]
pub enum Vis {
    Pub(keyword!("pub")),
    #[fallback]
    Priv,
}
