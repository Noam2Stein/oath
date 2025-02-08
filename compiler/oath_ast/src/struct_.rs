use crate::*;

pub struct Struct {
    pub vis: Vis,
    pub struct_keyword: keyword!("struct"),
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub group: Group<Braces>,
}

impl ItemParse for Struct {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
        tags: Vec<ItemTag>,
    ) -> Result<Self, ()> {
        let mut vis = Vis::Priv;

        for tag in tags {
            match tag {
                ItemTag::Pub(tag) => {
                    if let Vis::Priv = vis {
                        vis = Vis::Pub(tag)
                    } else {
                        diagnostics.push_error(Error::StaticMessage("multiple `pub`"), tag.span());
                    }
                }
                ItemTag::Con(tag) => diagnostics.push_error(
                    Error::StaticMessage("`con` cannot be put on structs"),
                    tag.span(),
                ),
                ItemTag::Raw(tag) => diagnostics.push_error(
                    Error::StaticMessage("`raw` cannot be put on structs"),
                    tag.span(),
                ),
            }
        }
    }
}
