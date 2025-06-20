use super::*;

pub enum Item {
    Struct(Struct),
    Error(Option<DiagnosticHandle>),
}

pub struct Mod {
    pub items: Vec<Item>,
    pub leftovers: Leftovers,
}

pub struct Struct {}

pub struct Field {
    pub ident: Ident,
    pub type_: oathc_ast::Expr,
    pub bounds: oathc_ast::Expr,
}

impl Item {
    pub fn from_ast(ast: oathc_ast::Item, diagnostics: &Diagnostics) -> Self {
        let core = match ast.core {
            Try::Failure(error) => return Self::Error(error),
            Try::Success(core) => core,
        };

        let core_span = match &core {
            oathc_ast::ItemCore::Attr(core) => core.hash.span(),
            oathc_ast::ItemCore::Enum(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Fn(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Mod(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Static(core) => core.keyword.span(),
            oathc_ast::ItemCore::Struct(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Sys(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Trait(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Type(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
            oathc_ast::ItemCore::Use(core) => core.keyword.span(),
        };

        match core {
            _ => Self::Error(Some(diagnostics.push_error(Error::ToDo(core_span)))),
        }
    }
}
