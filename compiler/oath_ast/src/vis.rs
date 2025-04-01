use crate::*;

#[derive(Debug, Clone, Parse, OptionDetect)]
#[desc = "a visibility"]
pub enum Vis {
    Pub(keyword!("pub")),
    #[fallback]
    Priv,
}
