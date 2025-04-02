use crate::*;

#[derive(Debug, Clone, Copy, Hash, Spanned, InternedDisplay)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

verify_literal_type!(Literal);

impl<'a> TryFrom<&'a TokenTree> for Literal {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(value) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

#[macro_export(local_inner_macros)]
macro_rules! verify_literal_type {
    ($type:ty) => {
        verify_token_type!($type);
        const _: () = verify_literal_type_helper::<$type>();
    };
}

#[allow(dead_code)]
pub(super) const fn verify_literal_type_helper<
    T: Debug + Copy + Eq + Ord + Hash + TryFrom<Literal> + Into<Literal> + Spanned,
>() {
}
