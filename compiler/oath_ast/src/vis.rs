use crate::*;

#[derive(Debug, Clone, Desc, Parse)]
#[desc = "a visibility"]
pub enum Vis {
    Pub(keyword!("pub")),
    Priv,
}
