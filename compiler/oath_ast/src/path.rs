use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a path"]
pub struct Path(pub Vec<PathSegment>);

#[derive(Debug, Clone, Desc)]
#[desc = "a path segment"]
pub enum PathSegment {
    Ident(Ident, GenericArgs),
    Package(keyword!("package")),
    Super(keyword!("super")),
}

impl TryParse for Path {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let segments = parser
            .try_parse_sep::<_, punct!("::")>(context)?
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        if segments.len() == 0 {
            return Err(());
        }

        Ok(Self(segments))
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

impl TryParse for PathSegment {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(value) = parser.parse(context) {
            Ok(Self::Package(value))
        } else if let Some(value) = parser.parse(context) {
            Ok(Self::Super(value))
        } else {
            Ok(Self::Ident(
                parser.try_parse(context)?,
                parser.parse(context),
            ))
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
            Self::Ident(a, b) => a.span().connect(b.span()),
            Self::Package(a) => a.span(),
            Self::Super(a) => a.span(),
        }
    }
}
