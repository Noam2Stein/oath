use std::{
    fmt::{self, Display, Formatter},
    mem::take,
};

use derive_more::Display;

use crate::*;

#[derive(Debug, Clone)]
pub enum Item {
    Fn(Func),
    Struct(Struct),
    Trait(Trait),
    Mod(Mod),
    Spec(Sys),
    Impl(Impl),
    Unknown,
}

#[derive(Debug, Clone, Default)]
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
    Func(keyword!("func")),
    Mod(keyword!("mod")),
    Use(keyword!("use")),
    Val(keyword!("val")),
    Alias(keyword!("alias")),
    Impl(keyword!("impl")),
}

#[derive(Debug, Clone)]
pub struct ItemKind {
    pub target_keywords: Vec<ItemKeyword>,
    pub base: ItemKeyword,
}

pub trait ItemType: Sized {
    fn add_modifiers(
        &mut self,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    );
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

    pub fn is_empty(&self) -> bool {
        self.pub_.is_none() && self.con.is_none() && self.raw.is_none()
    }
    pub fn is_not_empty(&self) -> bool {
        self.pub_.is_some() || self.con.is_some() || self.raw.is_some()
    }

    pub fn expect_empty(&self, context: ContextHandle, item_desc: &'static str) {
        if let Some(pub_) = self.pub_ {
            context.push_error(SyntaxError::CannotBeMarked(pub_.span(), "`pub`", item_desc));
        }
        if let Some(con) = self.con {
            context.push_error(SyntaxError::CannotBeMarked(con.span(), "`con`", item_desc));
        }
        if let Some(raw) = self.raw {
            context.push_error(SyntaxError::CannotBeMarked(raw.span(), "`raw`", item_desc));
        }
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

    fn parse_error() -> Self {
        Self {
            con: Parse::parse_error(),
            pub_: Parse::parse_error(),
            raw: Parse::parse_error(),
        }
    }
}

impl ItemKind {
    pub fn target(&self) -> Option<ItemKind> {
        self.target_keywords.last().map(|last_keyword| {
            let base = *last_keyword;

            let target_keywords = self
                .target_keywords
                .iter()
                .cloned()
                .take(self.target_keywords.len() - 1)
                .collect();

            Self {
                target_keywords,
                base,
            }
        })
    }

    pub fn expect_no_target(&self, context: ContextHandle) {
        if self.target_keywords.len() > 0 {
            context.push_error(Error::new(
                format!("SyntaxError: `{self}` is invalid"),
                self.span(),
            ));

            Try::Failure
        } else {
            Try::Success(())
        }
    }
}
impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for target_keyword in &self.target_keywords {
            write!(f, "{target_keyword}-")?;
        }

        write!(f, "{}", self.base)
    }
}
impl OptionParse for ItemKind {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let keywords = parser.option_parse_sep::<ItemKeyword, punct!("-")>()?;

        Some(Self {
            target_keywords: keywords.iter().cloned().take(keywords.len() - 1).collect(),
            base: *keywords.last(),
        })
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        ItemKeyword::detect(parser)
    }

    fn desc() -> &'static str {
        "an item kind"
    }
}
impl Spanned for ItemKind {
    fn span(&self) -> Span {
        self.target_keywords.first().map(|first| first.span()) + self.base.span()
    }
}

impl OptionParse for Item {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let mut modifiers = ItemModifiers::parse(parser);

        let item_kind = if modifiers.is_not_empty() {
            match ItemKind::try_parse(parser) {
                Try::Success(success) => success,
                Try::Failure => return Some(Self::Unknown),
            }
        } else {
            ItemKind::option_parse(parser)?
        };

        Some(match item_kind.base {
            ItemKeyword::Impl(_) => {
                Self::Impl(ItemType::item_parse(parser, &mut modifiers, item_kind))
            }
            ItemKeyword::Alias(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unknown
            }
            ItemKeyword::Val(keyword) => {
                parser.context().push_error(Error::new(
                    "`val` is not a standalone item-type",
                    keyword.span(),
                ));

                Self::Unknown
            }
            ItemKeyword::Const(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unknown
            }
            ItemKeyword::Enum(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unknown
            }
            ItemKeyword::Func(_) => {
                Self::Fn(ItemType::item_parse(parser, &mut modifiers, item_kind))
            }
            ItemKeyword::Mod(_) => {
                Self::Mod(ItemType::item_parse(parser, &mut modifiers, item_kind))
            }
            ItemKeyword::Sys(_) => {
                Self::Spec(ItemType::item_parse(parser, &mut modifiers, item_kind))
            }
            ItemKeyword::Static(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unknown
            }
            ItemKeyword::Struct(_) => {
                Self::Struct(ItemType::item_parse(parser, &mut modifiers, item_kind))
            }
            ItemKeyword::Trait(_) => {
                Self::Trait(ItemType::item_parse(parser, &mut modifiers, item_kind))
            }
            ItemKeyword::Type(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unknown
            }
            ItemKeyword::Use(keyword) => {
                parser
                    .context()
                    .push_error(Error::new("unfinished item type", keyword.span()));

                Self::Unknown
            }
        })
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        ItemModifiers::option_detect(parser) || ItemKind::detect(parser)
    }

    fn desc() -> &'static str {
        "an item"
    }
}
