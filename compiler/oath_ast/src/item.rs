use super::*;

// ITEM

#[derive(Debug, Clone, OptionParse)]
#[desc = "an item modifier"]
pub struct Item {
    pub attrs: Repeated<Attr>,
    pub modifiers: Repeated<ItemModifier>,
    pub base: Try<BaseItem>,
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

#[derive(Debug, Clone, OptionParse)]
#[desc = "an item"]
pub enum BaseItem {
    Mod(Mod),
    Use(Use),
    Fn(Fn),
    Struct(Struct),
    Enum(Enum),
    Sys(Sys),
    Static(Static),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`< >`"]
pub struct GenericParams {
    pub open: Discard<punct!("<")>,
    #[highlight(HighlightColor::Green)]
    pub values: List<Param>,
    pub close: Discard<Try<punct!(">")>>,
}

// MOD

#[derive(Debug, Clone, OptionParse)]
#[desc = "a module declaration"]
pub struct Mod {
    pub keyword: Discard<keyword!("mod")>,
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub content: Try<ModBlock>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum ModBlock {
    #[group]
    Block(delims!("{ }"), ModContent),
    Semi(Discard<punct!(";")>),
}

#[derive(Debug, Clone, Parse)]
pub struct ModContent {
    pub items: Repeated<Item>,
}

// USE

#[derive(Debug, Clone, OptionParse)]
#[desc = "a use item"]
pub struct Use {
    pub keyword: Discard<keyword!("use")>,
    pub target: Try<UseTarget>,
    pub semi: Try<punct!(";")>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a use target"]
pub enum UseTarget {
    Mod(keyword!("mod"), Try<Ident>),
}

// FUNC

#[derive(Debug, Clone, OptionParse)]
#[desc = "a function declaration"]
pub struct Fn {
    pub keyword: Discard<keyword!("fn")>,
    #[highlight(HighlightColor::Yellow)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub input: Try<FnInput>,
    pub output: Option<Expr<StrictBaseExpr>>,
    pub block: Try<FnBlock>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a function declaration"]
#[group]
pub struct FnInput {
    pub delims: delims!("( )"),
    #[highlight(HighlightColor::Cyan)]
    pub params: List<Param>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum FnBlock {
    Block(Block),
    Semi(Discard<punct!(";")>),
}

// STRUCT

#[derive(Debug, Clone, OptionParse)]
#[desc = "a struct declaration"]
pub struct Struct {
    pub keyword: keyword!("struct"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub fields: Try<Fields>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`{ }` / `()`"]
pub enum Fields {
    #[group]
    Named(delims!("{ }"), #[highlight(HighlightColor::Cyan)] List<Param>),
    #[group]
    Unnamed(delims!("( )"), List<UnnamedParam>),
}

// ENUM

#[derive(Debug, Clone, OptionParse)]
#[desc = "an enum declaration"]
pub struct Enum {
    pub keyword: keyword!("enum"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub variants: Try<Variants>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`{ }`"]
#[group]
pub struct Variants {
    pub delims: delims!("{ }"),
    pub variants: List<Variant>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a variant"]
pub struct Variant {
    #[highlight(HighlightColor::Green)]
    pub ident: Ident,
    pub fields: Option<Fields>,
}

// SYS

#[derive(Debug, Clone, OptionParse)]
#[desc = "a system declaration"]
pub struct Sys {
    pub keyword: Discard<keyword!("sys")>,
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub semi: Try<punct!(";")>,
}

// STATIC

#[derive(Debug, Clone, OptionParse)]
#[desc = "a static"]
pub struct Static {
    pub keyword: Discard<keyword!("static")>,
    pub mut_: Option<keyword!("mut")>,
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub type_: Option<Expr>,
    pub bounds: Option<Bounds>,
    pub eq: Try<Init>,
    pub semi: Try<punct!(";")>,
}
