use std::collections::HashMap;

use oath_ast::*;
use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_parse_context::*;
use oath_src::*;
use oath_tokens::*;

#[allow(private_bounds)]
pub trait NameresExt: Seal {
    fn nameres(self, context: &mut ParseContext);
}
trait Seal {}

impl Seal for SyntaxTree {}
impl NameresExt for SyntaxTree {
    fn nameres(self, context: &mut ParseContext) {
        let mut idents = Vec::new();
        let mut namespaces = Vec::new();

        {
            namespaces.push(Namespace::default());

            self.values.setup(0, &mut idents, &mut namespaces, context);
        }

        for (ident, namespace) in idents {
            namespaces[namespace].resolve(ident, context, &namespaces);
        }
    }
}

#[derive(Default)]
struct Namespace {
    parent: Option<usize>,
    names: HashMap<StrId, HighlightColor>,
}

impl Namespace {
    fn insert(&mut self, ident: Ident, color: HighlightColor, context: &mut ParseContext) {
        if self.names.contains_key(&ident.str_id()) {
            context.push_error(NameError::AlreadyExists(ident));
        } else {
            self.names.insert(ident.str_id(), color);
        }
    }

    fn resolve(&self, ident: Ident, context: &mut ParseContext, namespaces: &Vec<Namespace>) {
        if let Some(color) = self.names.get(&ident.str_id()) {
            context.highlight(ident, *color);
        } else if let Some(parent) = self.parent {
            namespaces[parent].resolve(ident, context, namespaces);
        } else {
            context.push_error(NameError::DoesntExist(ident));
        }
    }
}

trait Setup {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    );
}

impl<T: Setup> Setup for Vec<T> {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        for t in self {
            t.setup(namespace, idents, namespaces, context);
        }
    }
}

impl<T: Setup> Setup for Box<T> {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        T::setup(&self, namespace, idents, namespaces, context);
    }
}

impl<T: Setup> Setup for Try<T> {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        if let Try::Success(t) = self {
            t.setup(namespace, idents, namespaces, context);
        }
    }
}
impl<T: Setup> Setup for Option<T> {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        if let Some(t) = self {
            t.setup(namespace, idents, namespaces, context);
        }
    }
}

impl Setup for Ident {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        _namespaces: &mut Vec<Namespace>,
        _context: &mut ParseContext,
    ) {
        idents.push((*self, namespace));
    }
}

impl Setup for Item {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match &self.core {
            Try::Success(ItemCore::Attr(item)) => item.body.setup(namespace, idents, namespaces, context),
            Try::Success(ItemCore::Type(item)) => match item {
                TypeItem::Struct(item) => {
                    if let Try::Success(ident) = item.ident {
                        namespaces[namespace].insert(ident, HighlightColor::Green, context);
                    }

                    namespaces.push(Namespace {
                        parent: Some(namespace),
                        names: HashMap::new(),
                    });
                    let namespace = namespaces.len() - 1;

                    item.generics.setup(namespace, idents, namespaces, context);
                    item.contract.setup(namespace, idents, namespaces, context);
                    item.fields.setup(namespace, idents, namespaces, context);
                }
                TypeItem::Enum(item) => {
                    if let Try::Success(ident) = item.ident {
                        namespaces[namespace].insert(ident, HighlightColor::Green, context);
                    }

                    namespaces.push(Namespace {
                        parent: Some(namespace),
                        names: HashMap::new(),
                    });
                    let namespace = namespaces.len() - 1;

                    item.generics.setup(namespace, idents, namespaces, context);
                    item.contract.setup(namespace, idents, namespaces, context);
                    item.variants.setup(namespace, idents, namespaces, context);
                }
            },
            Try::Success(ItemCore::Fn(item)) => {
                if let Try::Success(ident) = item.ident {
                    namespaces[namespace].insert(ident, HighlightColor::Yellow, context);
                }

                namespaces.push(Namespace {
                    parent: Some(namespace),
                    names: HashMap::new(),
                });
                let namespace = namespaces.len() - 1;

                item.generics.setup(namespace, idents, namespaces, context);

                namespaces.push(Namespace {
                    parent: Some(namespace),
                    names: HashMap::new(),
                });
                let namespace = namespaces.len() - 1;

                item.input.setup(namespace, idents, namespaces, context);
                item.output.setup(namespace, idents, namespaces, context);
                item.contract.setup(namespace, idents, namespaces, context);
            }
            Try::Success(ItemCore::Mod(item)) => {
                if let Try::Success(ident) = item.ident {
                    namespaces[namespace].insert(ident, HighlightColor::Green, context);

                    item.body.setup(namespace, idents, namespaces, context);
                }
            }
            Try::Success(ItemCore::Static(item)) => {
                if let Try::Success(ident) = item.ident {
                    namespaces[namespace].insert(ident, HighlightColor::Green, context);
                }

                namespaces.push(Namespace {
                    parent: Some(namespace),
                    names: HashMap::new(),
                });
                let namespace = namespaces.len() - 1;

                item.generics.setup(namespace, idents, namespaces, context);
                item.contract.setup(namespace, idents, namespaces, context);
                item.eq.setup(namespace, idents, namespaces, context);
            }
            Try::Success(ItemCore::Sys(item)) => {
                if let Try::Success(ident) = item.ident {
                    namespaces[namespace].insert(ident, HighlightColor::Green, context);
                }

                namespaces.push(Namespace {
                    parent: Some(namespace),
                    names: HashMap::new(),
                });
                let namespace = namespaces.len() - 1;

                item.generics.setup(namespace, idents, namespaces, context);
                item.contract.setup(namespace, idents, namespaces, context);
            }
            Try::Success(ItemCore::Trait(item)) => {
                if let Try::Success(ident) = item.ident {
                    namespaces[namespace].insert(ident, HighlightColor::Green, context);
                }

                namespaces.push(Namespace {
                    parent: Some(namespace),
                    names: HashMap::new(),
                });
                let namespace = namespaces.len() - 1;

                item.generics.setup(namespace, idents, namespaces, context);
                item.contract.setup(namespace, idents, namespaces, context);
            }
            Try::Success(ItemCore::Use(item)) => item.body.setup(namespace, idents, namespaces, context),
            Try::Failure => {}
        }
    }
}

impl Setup for VarInit {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.init.setup(namespace, idents, namespaces, context);
    }
}

impl Setup for Variants {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.variants.values.setup(namespace, idents, namespaces, context);
    }
}
impl Setup for Variant {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.fields.setup(namespace, idents, namespaces, context);
    }
}

impl Setup for ModBody {
    fn setup(
        &self,
        _namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Block(_, items) => {
                let namespace = namespaces.len();
                namespaces.push(Namespace::default());

                items.values.setup(namespace, idents, namespaces, context);
            }
            Self::Semi(_) => {}
        }
    }
}

impl Setup for UseBody {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Mod(body) => {
                let span = body.ident.option_span().unwrap_or(body.keyword.span());

                match &body.body {
                    Try::Success(ModBody::Block(_, items)) => {
                        let mod_namespace = namespaces.len();
                        namespaces.push(Namespace::default());

                        items.values.setup(mod_namespace, idents, namespaces, context);

                        for (str_id, color) in namespaces[mod_namespace].names.clone() {
                            namespaces[namespace].insert(unsafe { Ident::from_id_unchecked(str_id, span) }, color, context);
                        }
                    }
                    Try::Success(ModBody::Semi(_)) => {
                        context.push_error(Error::Unfinished(span));
                    }
                    Try::Failure => {}
                }
            }
            Self::UsePath(path, _) => context.push_error(Error::Unfinished(path.span())),
        }
    }
}

impl Setup for AttrBody {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.ident.setup(namespace, idents, namespaces, context);
        self.value.setup(namespace, idents, namespaces, context);
    }
}

impl Setup for AttrInput {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            AttrInput::Eq(_, expr) => expr.setup(namespace, idents, namespaces, context),
            AttrInput::Parens(_, exprs) => exprs.values.setup(namespace, idents, namespaces, context),
        }
    }
}

impl Setup for Fields {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        namespaces.push(Namespace {
            parent: Some(namespace),
            names: HashMap::new(),
        });
        let namespace = namespaces.len() - 1;

        match self {
            Self::Named(_, params, contract) => {
                for param in &params.values {
                    namespaces[namespace].insert(param.name, HighlightColor::Cyan, context);

                    param.type_.setup(namespace, idents, namespaces, context);
                    param.bounds.setup(namespace, idents, namespaces, context);
                }

                contract.setup(namespace, idents, namespaces, context);
            }
            Self::Unnamed(_, params, contract) => {
                for param in &params.values {
                    param.type_.setup(namespace, idents, namespaces, context);
                    param.bounds.setup(namespace, idents, namespaces, context);
                }

                contract.setup(namespace, idents, namespaces, context);
            }
        }
    }
}

impl Setup for FnInput {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        for param in &self.params.values {
            namespaces[namespace].insert(param.name, HighlightColor::Cyan, context);

            param.type_.setup(namespace, idents, namespaces, context);
            param.bounds.setup(namespace, idents, namespaces, context);
        }
    }
}

impl Setup for Expr {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.lhs.setup(namespace, idents, namespaces, context);

        for rhs in &self.bin_ops.values {
            rhs.rhs.setup(namespace, idents, namespaces, context);
        }
    }
}
impl Setup for AngleExpr {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.lhs.setup(namespace, idents, namespaces, context);

        for rhs in &self.bin_ops.values {
            rhs.rhs.setup(namespace, idents, namespaces, context);
        }
    }
}
impl Setup for BraceExpr {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.lhs.setup(namespace, idents, namespaces, context);

        for rhs in &self.bin_ops.values {
            rhs.rhs.setup(namespace, idents, namespaces, context);
        }
    }
}
impl Setup for UnaryExpr {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.core.setup(namespace, idents, namespaces, context);
        self.postfix.values.setup(namespace, idents, namespaces, context);
    }
}
impl Setup for ExprPostfix {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Call(_, args) => args.values.setup(namespace, idents, namespaces, context),
            Self::Generics(_, args) => args.values.setup(namespace, idents, namespaces, context),
            Self::Index(_, args) => args.setup(namespace, idents, namespaces, context),
            Self::Member(_, _) => {}
        }
    }
}
impl Setup for BraceExprPostfix {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Call(_, args) => args.values.setup(namespace, idents, namespaces, context),
            Self::Generics(_, args) => args.values.setup(namespace, idents, namespaces, context),
            Self::Index(_, args) => args.setup(namespace, idents, namespaces, context),
            Self::Member(_, _) => {}
        }
    }
}

impl Setup for ExprCore {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Array(_, items) => items.values.setup(namespace, idents, namespaces, context),
            Self::Block(block) => {}
            Self::Ident(expr) => expr.setup(namespace, idents, namespaces, context),
            Self::If {
                keyword,
                condition,
                body,
            } => {}
            Self::Keyword(_) => {}
            Self::Literal(expr) => expr.setup(namespace, idents, namespaces, context),
            Self::Tuple(_, items) => items.values.setup(namespace, idents, namespaces, context),
        }
    }
}
impl Setup for UnaryBraceExpr {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.core.setup(namespace, idents, namespaces, context);
        self.postfix.values.setup(namespace, idents, namespaces, context);
    }
}
impl Setup for BraceExprCore {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Array(_, items) => items.values.setup(namespace, idents, namespaces, context),
            Self::Ident(expr) => expr.setup(namespace, idents, namespaces, context),
            Self::If {
                keyword,
                condition,
                body,
            } => {}
            Self::Keyword(_) => {}
            Self::Literal(expr) => expr.setup(namespace, idents, namespaces, context),
            Self::Tuple(_, items) => items.values.setup(namespace, idents, namespaces, context),
        }
    }
}
impl Setup for AngleUnaryExpr {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.core.setup(namespace, idents, namespaces, context);
        self.postfix.values.setup(namespace, idents, namespaces, context);
    }
}

impl Setup for Literal {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Literal::Int(lit) => lit.suffix.setup(namespace, idents, namespaces, context),
            Literal::Float(lit) => lit.suffix.setup(namespace, idents, namespaces, context),
            Literal::Char(_) => {}
            Literal::Str(_) => {}
        }
    }
}

impl Setup for GenericParams {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        for param in &self.values.values {
            namespaces[namespace].insert(param.name, HighlightColor::Green, context);

            param.type_.setup(namespace, idents, namespaces, context);
            param.bounds.setup(namespace, idents, namespaces, context);
        }
    }
}

impl Setup for Bounds {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.expr.setup(namespace, idents, namespaces, context);
    }
}

impl Setup for Contract {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.segments.values.setup(namespace, idents, namespaces, context);
    }
}
impl Setup for ContractSegment {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        match self {
            Self::Promise(_, body) => body.setup(namespace, idents, namespaces, context),
            Self::Require(_, body) => body.setup(namespace, idents, namespaces, context),
        }
    }
}
impl Setup for ContractBody {
    fn setup(
        &self,
        namespace: usize,
        idents: &mut Vec<(Ident, usize)>,
        namespaces: &mut Vec<Namespace>,
        context: &mut ParseContext,
    ) {
        self.items.values.setup(namespace, idents, namespaces, context);
    }
}
