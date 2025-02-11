use std::mem::take;

use crate::*;

pub enum Item {
    Fn(Fn),
    Struct(Struct),
    Mod(Mod),
}

#[derive(Debug, Clone, Default)]
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

    pub fn take_vis(&mut self) -> Vis {
        self.take_pub().map_or(Vis::Priv, |pub_| Vis::Pub(pub_))
    }

    fn expect_empty(&mut self, context: ContextHandle, item_desc: &'static str) {
        if let Some(pub_) = self.pub_ {
            context.push_error(SyntaxError::CannotBePutOn(pub_.span(), "`pub`", item_desc));
        }
        if let Some(con) = self.con {
            context.push_error(SyntaxError::CannotBePutOn(con.span(), "`con`", item_desc));
        }
        if let Some(raw) = self.raw {
            context.push_error(SyntaxError::CannotBePutOn(raw.span(), "`raw`", item_desc));
        }
    }
}

pub trait ItemType: Sized {
    const DESC: &str;

    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()>;

    fn item_peek(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool;

    fn item_parse_option(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Option<Result<Self, ()>> {
        if Self::item_peek(parser, context) {
            let output = Some(Self::item_parse(parser, context, modifiers));
            modifiers.expect_empty(context, Self::DESC);
            output
        } else {
            None
        }
    }
}

impl Parse for Item {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let mut modifiers = parser.parse::<ItemModifiers>(context).unwrap();

        if let Some(value) = ItemType::item_parse_option(parser, context, &mut modifiers) {
            Ok(Self::Fn(value?))
        } else if let Some(value) = ItemType::item_parse_option(parser, context, &mut modifiers) {
            Ok(Self::Struct(value?))
        } else if let Some(value) = ItemType::item_parse_option(parser, context, &mut modifiers) {
            Ok(Self::Mod(value?))
        } else {
            context.push_error(SyntaxError::Expected(parser.next_span(), "an item"));

            Err(())
        }
    }
}

impl Parse for ItemModifiers {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let mut output = Self::default();
        loop {
            if let Some(pub_) = parser.parse::<Option<keyword!("pub")>>(context).unwrap() {
                if output.pub_.is_some() {
                    context.push_error(SyntaxError::Double(pub_.span(), "`pub`"));
                } else {
                    output.pub_ = Some(pub_);
                }
            } else if let Some(con) = parser.parse::<Option<keyword!("con")>>(context).unwrap() {
                if output.con.is_some() {
                    context.push_error(SyntaxError::Double(con.span(), "`con`"));
                } else {
                    output.con = Some(con);
                }
            } else if let Some(raw) = parser.parse::<Option<keyword!("raw")>>(context).unwrap() {
                if output.raw.is_some() {
                    context.push_error(SyntaxError::Double(raw.span(), "`raw`"));
                } else {
                    output.raw = Some(raw);
                }
            } else {
                break;
            }
        }

        Ok(output)
    }
}
