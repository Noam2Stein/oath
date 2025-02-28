use std::mem::take;

use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "an item"]
pub enum Item {
    Fn(Fn),
    Struct(Struct),
    Mod(Mod),
}

#[derive(Debug, Clone, Default, Desc)]
#[desc = "item modifiers"]
pub struct ItemModifiers {
    pub_: Option<keyword!("pub")>,
    con: Option<keyword!("con")>,
    raw: Option<keyword!("raw")>,
}

pub trait ItemType: Peek + Desc {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()>;
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

impl TryParse for Item {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        fn parse_option_item<T: ItemType>(
            parser: &mut Parser<impl Iterator<Item = TokenTree>>,
            context: ContextHandle,
            modifiers: &mut ItemModifiers,
        ) -> Option<Result<T, ()>> {
            if parser.peek::<T>(context) {
                let output = Some(T::item_parse(parser, context, modifiers));
                modifiers.expect_empty(context, T::desc());
                output
            } else {
                None
            }
        }

        let mut modifiers = parser.parse::<ItemModifiers>(context);

        if let Some(value) = parse_option_item(parser, context, &mut modifiers) {
            Ok(Self::Fn(value?))
        } else if let Some(value) = parse_option_item(parser, context, &mut modifiers) {
            Ok(Self::Struct(value?))
        } else if let Some(value) = parse_option_item(parser, context, &mut modifiers) {
            Ok(Self::Mod(value?))
        } else {
            context.push_error(SyntaxError::Expected(parser.next_span(), "an item"));

            Err(())
        }
    }
}

impl Peek for Item {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<ItemModifiers>(context)
            || parser.peek::<Fn>(context)
            || parser.peek::<Struct>(context)
            || parser.peek::<Mod>(context)
    }
}

impl Parse for ItemModifiers {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let mut output = Self::default();
        loop {
            if let Some(pub_) = parser.parse::<Option<keyword!("pub")>>(context) {
                if output.pub_.is_some() {
                    context.push_error(SyntaxError::Double(pub_.span(), "`pub`"));
                } else {
                    output.pub_ = Some(pub_);
                }
            } else if let Some(con) = parser.parse::<Option<keyword!("con")>>(context) {
                if output.con.is_some() {
                    context.push_error(SyntaxError::Double(con.span(), "`con`"));
                } else {
                    output.con = Some(con);
                }
            } else if let Some(raw) = parser.parse::<Option<keyword!("raw")>>(context) {
                if output.raw.is_some() {
                    context.push_error(SyntaxError::Double(raw.span(), "`raw`"));
                } else {
                    output.raw = Some(raw);
                }
            } else {
                break;
            }
        }

        output
    }
}

impl Peek for ItemModifiers {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<keyword!("pub")>(context)
            || parser.peek::<keyword!("con")>(context)
            || parser.peek::<keyword!("raw")>(context)
    }
}
