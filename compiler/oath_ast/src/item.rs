use std::mem::take;

use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "an item"]
pub enum Item {
    Fn(Fn),
    Struct(Struct),
    Mod(Mod),
    Spec(Spec),
}

#[derive(Debug, Clone, Default, Desc)]
#[desc = "item modifiers"]
pub struct ItemModifiers {
    pub_: Option<keyword!("pub")>,
    con: Option<keyword!("con")>,
    raw: Option<keyword!("raw")>,
}

#[derive(Debug, Clone, Desc, Spanned, TryParse, Peek)]
#[desc = "an item-type"]
pub enum ItemKeyword {
    Struct(keyword!("struct")),
    Enum(keyword!("enum")),
    Type(keyword!("type")),
    Spec(keyword!("spec")),
    Trait(keyword!("trait")),
    Static(keyword!("static")),
    Const(keyword!("const")),
    Fn(keyword!("fn")),
    Mod(keyword!("mod")),
    Use(keyword!("use")),
    Val(keyword!("val")),
    Alias(keyword!("alias")),
}

#[derive(Debug, Clone, Desc)]
#[desc = "an item-type"]
pub struct ItemKind {
    pub keywords: Vec<ItemKeyword>,
}

pub trait ItemParse: Sized {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self>;
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

    pub fn expect_empty(&mut self, context: ContextHandle, item_desc: &'static str) {
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

impl PeekOk for ItemKeyword {}

impl TryParse for ItemKind {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        Ok(Self {
            keywords: parser
                .try_parse_sep::<_, punct!("-")>(context)?
                .into_iter()
                .collect::<PResult<_>>()?,
        })
    }
}
impl Peek for ItemKind {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<ItemKeyword>(context)
    }
}

impl Spanned for ItemKind {
    fn span(&self) -> Span {
        if self.keywords.len() == 0 {
            Span::end_of_file()
        } else {
            self.keywords
                .iter()
                .fold(self.keywords.first().unwrap().span(), |span, keyword| {
                    span.connect(keyword.span())
                })
        }
    }
}

impl ItemKind {
    pub fn expect_empty(&mut self, context: ContextHandle, item_desc: &'static str) {
        if self.keywords.len() != 0 {
            context.push_error(SyntaxError::CannotHaveTarget(self.span(), item_desc));
        }
    }
}

impl TryParse for Item {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let mut modifiers = parser.parse::<ItemModifiers>(context);
        let mut target_kind = parser.try_parse::<ItemKind>(context)?;

        match target_kind.keywords.pop().unwrap() {
            ItemKeyword::Alias(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
            ItemKeyword::Val(keyword) => {
                context.push_error(Error::new(
                    "`val` is not a standalone item-type",
                    keyword.span(),
                ));
                Err(())
            }
            ItemKeyword::Const(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
            ItemKeyword::Enum(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
            ItemKeyword::Fn(_) => Ok(Self::Fn(ItemParse::item_parse(
                parser,
                context,
                &mut modifiers,
                target_kind,
            )?)),
            ItemKeyword::Mod(_) => Ok(Self::Mod(ItemParse::item_parse(
                parser,
                context,
                &mut modifiers,
                target_kind,
            )?)),
            ItemKeyword::Spec(_) => Ok(Self::Spec(ItemParse::item_parse(
                parser,
                context,
                &mut modifiers,
                target_kind,
            )?)),
            ItemKeyword::Static(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
            ItemKeyword::Struct(_) => Ok(Self::Struct(ItemParse::item_parse(
                parser,
                context,
                &mut modifiers,
                target_kind,
            )?)),
            ItemKeyword::Trait(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
            ItemKeyword::Type(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
            ItemKeyword::Use(keyword) => {
                context.push_error(Error::new("unfinished item type", keyword.span()));
                Err(())
            }
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
