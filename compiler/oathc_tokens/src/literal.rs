use super::*;

#[derive(Debug, Spanned)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

const _: () = verify_token_type::<Literal>();

#[derive(Debug)]
pub struct LiteralSuffix {
    pub ident: Try<Ident>,
}
