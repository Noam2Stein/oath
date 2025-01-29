use oath_diagnostics::{Desc, DiagnosticsHandle, Error, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Ident, Literal, LiteralType, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntLiteral {
    int: u128,
    suffix: Option<Ident>,
    span: Span,
}

impl LiteralType for IntLiteral {}
impl TokenType for IntLiteral {}
impl Seal for IntLiteral {}
impl Spanned for IntLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}
impl Fill for IntLiteral {
    fn fill(span: Span) -> Self {
        Self::new(1, None, span)
    }
}
impl Desc for IntLiteral {
    fn desc() -> &'static str {
        "a float literal"
    }
}
impl TryFrom<Literal> for IntLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Int(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a Literal> for &'a IntLiteral {
    type Error = ();

    fn try_from(value: &'a Literal) -> Result<Self, Self::Error> {
        if let Literal::Int(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl TryFrom<TokenTree> for IntLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Int(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for &'a IntLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Int(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}

impl IntLiteral {
    #[inline(always)]
    pub fn new(int: u128, suffix: Option<Ident>, span: Span) -> Self {
        Self { int, suffix, span }
    }

    #[inline(always)]
    pub fn int(self) -> u128 {
        self.int
    }
    #[inline(always)]
    pub fn suffix(self) -> Option<Ident> {
        self.suffix
    }

    pub unsafe fn from_regex_str(str: &str, span: Span, diagnostics: DiagnosticsHandle) -> Self {
        let suffix_start = str
            .char_indices()
            .find(|(_, char)| !char.is_ascii_digit() && *char != '_')
            .map(|(char_pos, _)| char_pos);

        let int_str = &str[0..suffix_start.unwrap_or(str.len())].replace("_", "");
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let int = u128::from_str_radix(int_str, 10).unwrap_or_else(|_| {
            diagnostics.push_error(Error::StaticMessage("out of bounds literal"), span);
            1
        });

        let suffix = suffix_str.map_or(None, |suffix_str| {
            Ident::new(suffix_str.to_string(), span).or_else(|| {
                diagnostics.push_error(
                    Error::StaticMessage("expected an ident. found a keyword"),
                    span,
                );

                None
            })
        });

        Self { int, suffix, span }
    }
}
