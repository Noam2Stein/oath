use crate::*;

pub struct Path(pub Vec<PathSegment>);

pub enum PathSegment {
    Ident(Ident, Result<GenericArgs, ()>),
    Package(keyword!("package")),
    Super(keyword!("super")),
}

impl Parse for Path {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        Ok(Self(
            parser.parse_sep::<_, punct!("::"), true, false>(context)?,
        ))
    }
}
impl Peek for Path {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        PathSegment::peek(parser, context)
    }
}

impl Spanned for Path {
    fn span(&self) -> Span {
        self.0
            .iter()
            .fold(self.0.first().unwrap().span(), |prev, seg| {
                seg.span().connect(prev)
            })
    }
}

impl Parse for PathSegment {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(value) = parser.parse(context).unwrap() {
            Ok(Self::Package(value))
        } else if let Some(value) = parser.parse(context).unwrap() {
            Ok(Self::Super(value))
        } else {
            Ok(Self::Ident(parser.parse(context)?, parser.parse(context)))
        }
    }
}

impl Peek for PathSegment {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
            || parser.peek::<keyword!("package")>(context)
            || parser.peek::<keyword!("super")>(context)
    }
}

impl Spanned for PathSegment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(a, b) => b.as_ref().map_or(a.span(), |b| a.span().connect(b.span())),
            Self::Package(a) => a.span(),
            Self::Super(a) => a.span(),
        }
    }
}
