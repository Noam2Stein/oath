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
    Spec(Sys),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`< >`"]
pub struct GenericParams(punct!("<"), pub Trailing<Param, punct!(",")>, Try<punct!(">")>);

// MOD

#[derive(Debug, Clone, OptionParse)]
#[desc = "a module declaration"]
pub struct Mod {
    pub _keyword: Discard<keyword!("mod")>,
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
    pub _keyword: Discard<keyword!("fn")>,
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
    pub _delims: delims!("( )"),
    pub params: Trailing<Param, punct!(",")>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "output try"]
pub struct FuncOutput {
    pub _arrow: Discard<punct!("->")>,
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
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub fields: Try<StructFields>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`{ }` / `()`"]
pub enum StructFields {
    #[group]
    Named(delims!("{ }"), Trailing<Param, punct!(",")>),
    #[group]
    Unnamed(delims!("( )"), Trailing<UnnamedParam, punct!(",")>),
}

// SYS

#[derive(Debug, Clone, OptionParse)]
#[desc = "a system declaration"]
pub struct Sys {
    pub _keyword: Discard<keyword!("sys")>,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub _semi: Try<punct!(";")>,
}
