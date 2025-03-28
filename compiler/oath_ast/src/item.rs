use std::mem::take;

use derive_more::Display;
use nonempty::NonEmpty;

use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "an item"]
pub enum Item {
    Fn(Fn),
    Struct(Struct),
    Trait(Trait),
    Mod(Mod),
    Spec(Sys),
    Impl(Impl),
    Unfinished,
}

#[derive(Debug, Clone, Default, ParseDesc)]
#[desc = "item modifiers"]
pub struct ItemModifiers {
    pub_: Option<keyword!("pub")>,
    con: Option<keyword!("con")>,
    raw: Option<keyword!("raw")>,
}

#[derive(Debug, Clone, Copy, Display, Spanned, OptionParse)]
#[desc = "an item-type"]
pub enum ItemKeyword {
    Struct(keyword!("struct")),
    Enum(keyword!("enum")),
    Type(keyword!("type")),
    Sys(keyword!("sys")),
    Trait(keyword!("trait")),
    Static(keyword!("static")),
    Const(keyword!("const")),
    Fn(keyword!("fn")),
    Mod(keyword!("mod")),
    Use(keyword!("use")),
    Val(keyword!("val")),
    Alias(keyword!("alias")),
    Impl(keyword!("impl")),
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "an item-type"]
pub struct ItemKind {
    pub keywords: NonEmpty<ItemKeyword>,
}

pub trait ItemParse: Sized {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        target_kind: Option<ItemKind>,
        kind_keyword: ItemKeyword,
    ) -> Self;
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

impl OptionParse for ItemKind {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        parser
            .option_parse_sep::<_, punct!("-")>()
            .map(|keywords| Self { keywords })
    }
}
impl Detect for ItemKind {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        ItemKeyword::detect(parser)
    }
}
impl Spanned for ItemKind {
    fn span(&self) -> Span {
        self.keywords
            .iter()
            .fold(self.keywords.first().span(), |span, keyword| {
                span + keyword.span()
            })
    }
}
impl ItemKind {
    pub fn expect_empty(self, context: ContextHandle, item_desc: &'static str) {
        if self.keywords.len() != 0 {
            context.push_error(SyntaxError::CannotHaveTarget(self.span(), item_desc));
        }
    }
}

impl OptionParse for Item {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let mut modifiers = ItemModifiers::parse(parser);
        let mut target_kind = ItemKind::option_parse(parser)?;

        let kind_keyword = *target_kind.keywords.last();
        let target_kind = if target_kind.keywords.len() > 1 {
            target_kind.keywords.pop();
            Some(target_kind)
        } else {
            None
        };

        Some(match kind_keyword {
            ItemKeyword::Impl(_) => Self::Impl(ItemParse::item_parse(
                parser,
                &mut modifiers,
                target_kind,
                kind_keyword,
            )),
            ItemKeyword::Alias(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unfinished
            }
            ItemKeyword::Val(keyword) => {
                parser.context().push_error(Error::new(
                    "`val` is not a standalone item-type",
                    keyword.span(),
                ));

                Self::Unfinished
            }
            ItemKeyword::Const(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unfinished
            }
            ItemKeyword::Enum(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unfinished
            }
            ItemKeyword::Fn(_) => Self::Fn(ItemParse::item_parse(
                parser,
                &mut modifiers,
                target_kind,
                kind_keyword,
            )),
            ItemKeyword::Mod(_) => Self::Mod(ItemParse::item_parse(
                parser,
                &mut modifiers,
                target_kind,
                kind_keyword,
            )),
            ItemKeyword::Sys(_) => Self::Spec(ItemParse::item_parse(
                parser,
                &mut modifiers,
                target_kind,
                kind_keyword,
            )),
            ItemKeyword::Static(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unfinished
            }
            ItemKeyword::Struct(_) => Self::Struct(ItemParse::item_parse(
                parser,
                &mut modifiers,
                target_kind,
                kind_keyword,
            )),
            ItemKeyword::Trait(_) => Self::Trait(ItemParse::item_parse(
                parser,
                &mut modifiers,
                target_kind,
                kind_keyword,
            )),
            ItemKeyword::Type(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unfinished
            }
            ItemKeyword::Use(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unfinished
            }
        })
    }
}

impl Detect for Item {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        ItemModifiers::detect(parser) || ItemKind::detect(parser)
    }
}

impl Parse for ItemModifiers {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let mut output = Self::default();
        loop {
            if let Some(keyword) = Parse::parse(parser) {
                if output.pub_.is_none() {
                    output.pub_ = Some(keyword);
                } else {
                    parser
                        .context()
                        .push_error(SyntaxError::Double(keyword.span(), "`pub`"));
                }
            } else if let Some(keyword) = Parse::parse(parser) {
                if output.con.is_none() {
                    output.con = Some(keyword);
                } else {
                    parser
                        .context()
                        .push_error(SyntaxError::Double(keyword.span(), "`pub`"));
                }
            } else if let Some(keyword) = Parse::parse(parser) {
                if output.raw.is_none() {
                    output.raw = Some(keyword);
                } else {
                    parser
                        .context()
                        .push_error(SyntaxError::Double(keyword.span(), "`pub`"));
                }
            } else {
                break;
            }
        }

        output
    }
}

impl Detect for ItemModifiers {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        <keyword!("pub")>::detect(parser)
            || <keyword!("con")>::detect(parser)
            || <keyword!("raw")>::detect(parser)
    }
}
