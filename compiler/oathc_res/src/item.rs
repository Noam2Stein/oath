use std::path::Path;

use super::*;

#[derive(Debug)]
pub enum Item {
    Mod(ItemMod),
    Error(oathc_ast::Item),
    ToDo(Option<DiagnosticHandle>, oathc_ast::Item),
}

impl Item {
    pub fn new(
        ast: oathc_ast::Item,
        submod_dir: Option<&Path>,
        interner: &Interner,
        file_interner: &FileInterner,
        diagnostics: &Diagnostics,
    ) -> Self {
        match ast.core {
            Try::Success(oathc_ast::ItemCore::Mod(core)) => {
                Self::Mod(ItemMod::new(core, submod_dir, interner, file_interner, diagnostics))
            }
            Try::Success(ref core) => {
                let _core_span = match core {
                    oathc_ast::ItemCore::Attr(core) => core.hash.span(),
                    oathc_ast::ItemCore::Enum(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
                    oathc_ast::ItemCore::Fn(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
                    oathc_ast::ItemCore::Mod(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
                    oathc_ast::ItemCore::Static(core) => core.keyword.span(),
                    oathc_ast::ItemCore::Struct(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
                    oathc_ast::ItemCore::Sys(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
                    oathc_ast::ItemCore::Trait(core) => core.ident.option_span().unwrap_or(core.keyword.span()),
                    oathc_ast::ItemCore::Use(core) => core.keyword.span(),
                };

                Self::ToDo(None, ast)
            }
            Try::Failure(_) => Self::Error(ast),
        }
    }
}
