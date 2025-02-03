use std::mem::replace;

use oath_diagnostics::{Desc, DiagnosticsHandle, Error};
use oath_parser::{parse_garbage, Garbage, Parse, Parser, Peek, RepEndless};
use oath_tokenizer::{keyword, TokenTree};

use crate::{Fn, Mod, Pub, Struct, Use};

#[derive(Parse)]
pub struct ModContent {
    pub items: RepEndless<ModItem>,
}

#[derive(Peek, Desc)]
#[desc("an item")]
pub enum ModItem {
    Mod(Mod),
    Use(Use),
    Struct(Struct),
    Fn(Fn),
    #[dont_peek]
    Garbage(Garbage<Self>),
}

impl Parse for ModItem {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let attribs = parser.parse::<Vec<ModItemAttrib>>(diagnostics);

        if let Some(item) = ItemParse::item_parse_option(parser, diagnostics, &mut attribs) {
            Self::Mod(item)
        } else if let Some(item) = ItemParse::item_parse_option(parser, diagnostics, &mut attribs) {
            Self::Use(item)
        } else if let Some(item) = ItemParse::item_parse_option(parser, diagnostics, &mut attribs) {
            Self::Struct(item)
        } else if let Some(item) = ItemParse::item_parse_option(parser, diagnostics, &mut attribs) {
            Self::Fn(item)
        } else {
            diagnostics.push_error(
                Error::Expected("an item"),
                parse_garbage(parser, |parser| {
                    parser.peek::<ModItem>() | parser.peek::<ModItemAttrib>()
                }),
            );

            Self::Garbage(Default::default())
        }
    }
}

pub trait ItemParse: Sized + Peek {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
        attribs: Vec<ModItemAttrib>,
    ) -> Self;

    fn item_parse_option(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
        attribs: &mut Vec<ModItemAttrib>,
    ) -> Option<Self> {
        if Self::peek(parser) {
            Some(Self::item_parse(
                parser,
                diagnostics,
                replace(attribs, Vec::new()),
            ))
        } else {
            None
        }
    }
}

#[derive(Parse, Peek)]
pub enum ModItemAttrib {
    Pub(Pub),
    Raw(keyword!("raw")),
    Con(keyword!("con")),
}
