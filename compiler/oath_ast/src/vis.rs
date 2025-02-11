use crate::*;

pub enum Vis {
    Pub(keyword!("pub")),
    Priv,
}

impl Parse for Vis {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(pub_) = parser.parse(context).unwrap() {
            Ok(Self::Pub(pub_))
        } else {
            Ok(Self::Priv)
        }
    }
}
