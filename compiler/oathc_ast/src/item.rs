use super::*;

#[derive(Debug, OptionParse)]
#[desc = "an item"]
pub struct Item {
    #[parse_as(Repeated<Attr>)]
    pub attrs: Vec<Attr>,
    #[parse_as(Repeated<ItemModifier>)]
    pub modifiers: Vec<ItemModifier>,
    pub core: Try<ItemCore>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an item modifier"]
pub enum ItemModifier {
    Pub(keyword!("pub")),
    Open(keyword!("open")),
    Runtime(keyword!("runtime")),
    Comptime(keyword!("comptime")),
    Raw(keyword!("raw")),
    Con(keyword!("con")),
}

#[derive(Debug, OptionParse)]
#[desc = "an item"]
pub enum ItemCore {
    Attr(InnerAttr),
    Mod(Mod),
    Use(Use),
    Fn(Fn),
    Struct(Struct),
    Enum(Enum),
    Sys(Sys),
    Static(Static),
    Trait(Trait),
}

// Mod

#[derive(Debug, OptionParse)]
#[desc = "a module declaration"]
pub struct Mod {
    pub keyword: keyword!("mod"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub body: Try<ModBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum ModBody {
    Block(ModBlock),
    Semi(punct!(";")),
}

#[derive(Debug, OptionParse)]
#[desc = "`{ }`"]
#[framed]
pub struct ModBlock {
    pub frame: Frame<delims!("{ }")>,
    #[parse_as(Repeated<_>)]
    pub items: Vec<Item>,
}

// Use

#[derive(Debug, OptionParse)]
#[desc = "a use statement"]
pub struct Use {
    pub keyword: keyword!("use"),
    pub body: Try<UseBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "a use target"]
pub enum UseBody {
    Mod(Mod),
    UsePath(UsePath, Try<punct!(";")>),
}

#[derive(Debug, OptionParse)]
#[desc = "an import path"]
pub enum UsePath {
    Ident(Ident, Option<UseExt>),
    Parent(keyword!("parent"), Option<UseExt>),
    All(punct!("*")),
    List(UseList),
}

#[derive(Debug, OptionParse)]
#[desc = "`.`"]
pub struct UseExt {
    pub dot: punct!("."),
    pub members: Box<Try<UsePath>>,
}

#[derive(Debug, OptionParse)]
#[desc = "`{ }`"]
#[framed]
pub struct UseList {
    pub frame: Frame<delims!("{ }")>,
    #[parse_as(Trailing<_, punct!(",")>)]
    pub paths: Vec<UsePath>,
}

// Fn

#[derive(Debug, OptionParse)]
#[desc = "a function declaration"]
pub struct Fn {
    pub keyword: keyword!("fn"),
    #[highlight(HighlightColor::Yellow)]
    pub ident: Try<Ident>,
    pub generics: Option<FramedParams<Angles>>,
    pub input: Try<FramedParams<delims!("( )")>>,
    pub output: Option<BraceExpr>,
    pub contract: Contract,
    pub body: Try<FnBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum FnBody {
    Block(Block),
    Semi(punct!(";")),
}

// Sys

#[derive(Debug, OptionParse)]
#[desc = "a system declaration"]
pub struct Sys {
    pub keyword: keyword!("sys"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<FramedParams<Angles>>,
    pub contract: Contract,
    pub semi: Try<punct!(";")>,
}

// Static

#[derive(Debug, OptionParse)]
#[desc = "a static"]
pub struct Static {
    pub keyword: keyword!("static"),
    #[highlight(HighlightColor::Blue)]
    pub param: Try<Param>,
    pub value: Option<Assign>,
    pub semi: Try<punct!(";")>,
}

// Trait

#[derive(Debug, OptionParse)]
#[desc = "a trait"]
pub struct Trait {
    pub keyword: keyword!("trait"),
    pub mut_: Option<keyword!("mut")>,
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<FramedParams<Angles>>,
    pub target: Option<TraitTarget>,
    pub contract: Contract,
    pub body: Try<ModBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "a trait target type"]
#[framed]
pub struct TraitTarget {
    pub frame: Frame<delims!("( )")>,
    pub target: Try<Expr>,
}
