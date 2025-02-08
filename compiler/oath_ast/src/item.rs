use std::mem::take;

use crate::*;

pub enum Item {}

pub struct ItemModifiers {
    pub_: Option<keyword!("pub")>,
    con: Option<keyword!("con")>,
    raw: Option<keyword!("raw")>,
}

impl ItemModifiers {
    pub fn take_pub(&mut self) -> Option<keyword!("pub")> {
        take(&mut self.pub_)
    }
    pub fn take_con(&mut self) -> Option<keyword!("con")> {
        take(&mut self.con)
    }
    pub fn take_raw(&mut self) -> Option<keyword!("raw")> {
        take(&mut self.raw)
    }

    pub fn expect_empty(self, diagnostics: DiagnosticsHandle) {
        if let Some(pub_) = self.pub_ {
            diagnostics.push_error(Error::StaticMessage(""), span);
        }
    }
}

pub trait ItemParse: Sized {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
        modifiers: ItemModifiers,
    ) -> Result<Self, ()>;
}
