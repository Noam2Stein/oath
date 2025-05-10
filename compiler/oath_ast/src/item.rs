use super::*;

// ITEM

#[derive(Debug, Clone, OptionParse)]
#[desc = "an item modifier"]
pub struct Item {
    pub modifiers: Repeated<ItemModifier>,
    pub base: Try<BaseItem>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an item modifier"]
pub enum ItemModifier {
    Pub(keyword!("pub")),
    Open(keyword!("open")),
    Raw(keyword!("raw")),
    Con(keyword!("con")),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an item"]
pub enum BaseItem {
    Mod(Mod),
    Fn(Func),
    Struct(Struct),
    Sys(Sys),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`< >`"]
pub struct GenericParams {
    #[highlight(HighlightColor::Blue)]
    pub open: punct!("<"),
    #[highlight(HighlightColor::Green)]
    pub values: Trailing<Param, punct!(",")>,
    #[highlight(HighlightColor::Blue)]
    pub close: Try<punct!(">")>,
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

// FUNC

#[derive(Debug, Clone, OptionParse)]
#[desc = "a function declaration"]
pub struct Func {
    pub keyword: Discard<keyword!("fn")>,
    #[highlight(HighlightColor::Yellow)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub input: Try<FuncInput>,
    pub output: Option<FuncOutput>,
    pub block: Try<FuncBlock>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a function declaration"]
#[group]
pub struct FuncInput {
    pub delims: delims!("( )"),
    #[highlight(HighlightColor::Cyan)]
    pub params: Trailing<Param, punct!(",")>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "output try"]
pub struct FuncOutput {
    pub arrow: Discard<punct!("->")>,
    pub type_: Try<Expr>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum FuncBlock {
    #[group]
    Block(delims!("{ }"), Trailing<Stmt, punct!(";")>),
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
    pub fields: Try<StructFields>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`{ }` / `()`"]
pub enum StructFields {
    #[group]
    Named(
        delims!("{ }"),
        #[highlight(HighlightColor::Cyan)] Trailing<Param, punct!(",")>,
    ),
    #[group]
    Unnamed(delims!("( )"), Trailing<UnnamedParam, punct!(",")>),
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
