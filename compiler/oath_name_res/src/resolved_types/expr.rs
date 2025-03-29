use oath_ast::ItemKind;
use oath_context::ContextHandle;
use oath_parser::Try;
use oath_src::{Span, Spanned};
use oath_tokenizer::Literal;

use crate::{DumbNameContext, NameId, NamespaceId};

#[derive(Debug, Clone, Spanned)]
pub enum Expr {
    Name(#[span] Span, Try<NameId>),
    Literal(Literal),
    ItemKind(ItemKind),
    Tuple(#[span] Span, Vec<Try<Expr>>),
    Array(#[span] Span, Vec<Try<Expr>>),
    Block(oath_ast::Block),
    Field(Box<Expr>, oath_ast::FieldIdent),
    Index(#[span] Span, Box<Expr>, Try<Box<Expr>>),
    Call(#[span] Span, Box<Expr>, Vec<Try<Expr>>),
    Generics(Box<Expr>, oath_ast::GenericArgs),
    ShsOp(#[span] Span, oath_ast::ShsOp, Try<Box<Expr>>),
    MhsOp(#[span] Span, Box<Expr>, oath_ast::MhsOp, Try<Box<Expr>>),
}

impl Expr {
    pub fn from_syntax(
        ast_expr: oath_ast::Expr,
        namespace: NamespaceId,
        name_context: &DumbNameContext,
        context: ContextHandle,
    ) -> Self {
        match ast_expr {
            oath_ast::Expr::Ident(ident) => Self::Name(
                ident.span(),
                name_context
                    .namespace(namespace)
                    .try_name_id(ident, context),
            ),
            oath_ast::Expr::Literal(expr) => Self::Literal(expr),
            oath_ast::Expr::ItemKind(expr) => Self::ItemKind(expr),
            oath_ast::Expr::Tuple(span, items) => Self::Tuple(
                span,
                items
                    .into_iter()
                    .map(|item| {
                        item.map(|item| Self::from_syntax(item, namespace, name_context, context))
                    })
                    .collect(),
            ),
            oath_ast::Expr::Array(span, items) => Self::Array(
                span,
                items
                    .into_iter()
                    .map(|item| {
                        item.map(|item| Self::from_syntax(item, namespace, name_context, context))
                    })
                    .collect(),
            ),
            oath_ast::Expr::Block(expr) => Self::Block(expr),
            oath_ast::Expr::Field(base, field) => Self::Field(
                Box::new(Self::from_syntax(*base, namespace, name_context, context)),
                field,
            ),
            oath_ast::Expr::Index(span, base, index) => Self::Index(
                span,
                Box::new(Self::from_syntax(*base, namespace, name_context, context)),
                index.map(|index| {
                    Box::new(Self::from_syntax(*index, namespace, name_context, context))
                }),
            ),
            oath_ast::Expr::Call(span, base, args) => Self::Call(
                span,
                Box::new(Self::from_syntax(*base, namespace, name_context, context)),
                args.into_iter()
                    .map(|arg| {
                        arg.map(|arg| Self::from_syntax(arg, namespace, name_context, context))
                    })
                    .collect(),
            ),
            oath_ast::Expr::Generics(base, generics) => Self::Generics(
                Box::new(Self::from_syntax(*base, namespace, name_context, context)),
                generics,
            ),
            oath_ast::Expr::ShsOp(span, op, base) => Self::ShsOp(
                span,
                op,
                base.map(|base| {
                    Box::new(Self::from_syntax(*base, namespace, name_context, context))
                }),
            ),
            oath_ast::Expr::MhsOp(span, lhs, op, rhs) => Self::MhsOp(
                span,
                Box::new(Self::from_syntax(*lhs, namespace, name_context, context)),
                op,
                rhs.map(|rhs| Box::new(Self::from_syntax(*rhs, namespace, name_context, context))),
            ),
        }
    }
}
