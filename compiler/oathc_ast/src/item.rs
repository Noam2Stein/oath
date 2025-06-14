use super::*;

#[derive(Debug, OptionParse)]
#[desc = "an item"]
pub struct Item {
    pub attrs: Repeated<Attr>,
    pub modifiers: Repeated<ItemModifier>,
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
    Type(Type),
    Struct(Struct),
    Enum(Enum),
    Sys(Sys),
    Static(Static),
    Trait(Trait),
}

// MOD

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
    #[framed]
    Block(delims!("{ }"), Repeated<Item>, Leftovers),
    Semi(punct!(";")),
}

// USE

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

#[derive(Debug, OptionParse, Spanned)]
#[desc = "an import path"]
pub enum UsePath {
    Ident(#[span] Ident, Option<UseDot>),
    Parent(#[span] keyword!("parent"), Option<UseDot>),
    All(punct!("*")),
    #[framed]
    List(delims!("{ }"), List<UsePath>, Leftovers),
}

#[derive(Debug, OptionParse)]
#[desc = "`.`"]
pub struct UseDot {
    pub dot: punct!("."),
    pub members: Box<Try<UsePath>>,
}

// FN

#[derive(Debug, OptionParse)]
#[desc = "a function declaration"]
pub struct Fn {
    pub keyword: keyword!("fn"),
    #[highlight(HighlightColor::Yellow)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub input: Try<FnInput>,
    pub output: Option<BraceExpr>,
    pub contract: Contract,
    pub body: Try<FnBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "a function declaration"]
#[framed]
pub struct FnInput {
    pub delims: delims!("( )"),
    #[highlight(HighlightColor::Cyan)]
    pub params: List<Param>,
    pub leftovers: Leftovers,
}

#[derive(Debug, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum FnBody {
    Block(Block),
    Semi(punct!(";")),
}

// Type

#[derive(Debug, OptionParse)]
#[desc = "a type declaration"]
pub struct Type {
    pub keyword: keyword!("type"),
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub semi: Try<punct!(";")>,
}

#[derive(Debug, OptionParse)]
#[desc = "a struct declaration"]
pub struct Struct {
    pub keyword: keyword!("struct"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub fields: Try<TypeMembers>,
}

#[derive(Debug, OptionParse)]
#[desc = "an enum declaration"]
pub struct Enum {
    pub keyword: keyword!("enum"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub variants: Try<TypeMembers>,
}

#[derive(Debug, OptionParse)]
#[desc = "`{ }` / `()`"]
pub enum TypeMembers {
    #[framed]
    Named(
        delims!("{ }"),
        #[highlight(HighlightColor::Cyan)] List<Param>,
        Contract,
        Leftovers,
    ),
    #[framed]
    Unnamed(delims!("( )"), List<UnnamedParam>, Contract, Leftovers),
}

// SYS

#[derive(Debug, OptionParse)]
#[desc = "a system declaration"]
pub struct Sys {
    pub keyword: keyword!("sys"),
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub semi: Try<punct!(";")>,
}

// STATIC

#[derive(Debug, OptionParse)]
#[desc = "a static"]
pub struct Static {
    pub keyword: keyword!("static"),
    pub mut_: Option<keyword!("mut")>,
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub type_: Option<Expr>,
    pub bounds: Option<Bounds>,
    pub contract: Contract,
    pub eq: Try<VarInit>,
    pub semi: Try<punct!(";")>,
}

// TRAIT

#[derive(Debug, OptionParse)]
#[desc = "a trait"]
pub struct Trait {
    pub keyword: keyword!("trait"),
    pub mut_: Option<keyword!("mut")>,
    #[highlight(HighlightColor::Green)]
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub target: Option<TraitTarget>,
    pub contract: Contract,
    pub body: Try<ApiBody>,
}

#[derive(Debug, OptionParse)]
#[desc = "a trait target type"]
#[framed]
pub struct TraitTarget {
    pub frame: delims!("( )"),
    pub target: Try<Expr>,
    pub leftovers: Leftovers,
}

// PARAM

#[derive(Debug, Spanned, OptionParse)]
#[desc = "a parameter"]
pub struct Param {
    #[span]
    pub name: Ident,
    pub type_: Option<AngleUnaryExpr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, OptionParse)]
#[desc = "an unnamed parameter"]
pub struct UnnamedParam {
    pub type_: AngleUnaryExpr,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    pub colon: punct!(":"),
    pub expr: Try<Expr>,
}

impl Highlightable for Param {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        self.name.highlight(color, h);
    }
}

// GENERICS

#[derive(Debug, OptionParse)]
#[desc = "`< >`"]
#[framed]
pub struct GenericParams {
    pub frame: Angles,
    #[highlight(HighlightColor::Green)]
    pub values: List<Param>,
    pub leftovers: Leftovers,
}

// API

#[derive(Debug, OptionParse)]
#[desc = "either `{ } or `;`"]
pub enum ApiBody {
    #[framed]
    Block(delims!("{ }"), Repeated<Item>, Leftovers),
    Semi(punct!(";")),
}

// CONTRACT

#[derive(Debug, Parse)]
pub struct Contract {
    pub segments: Repeated<ContractSegment>,
}

#[derive(Debug, OptionParse)]
#[desc = "a contract segment"]
pub enum ContractSegment {
    Require(keyword!("require"), Try<ContractBody>),
    Promise(keyword!("promise"), Try<ContractBody>),
}

#[derive(Debug, OptionParse)]
#[desc = "`[ ]`"]
#[framed]
pub struct ContractBody {
    pub delims: delims!("[ ]"),
    pub items: List<Expr>,
    pub leftovers: Leftovers,
}
