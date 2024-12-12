use super::*;

#[derive(Debug, Clone, Hash, Parse)]
pub struct Var {
    pub mut_token: Option<Keyword!("mut")>,
    pub var_token: Keyword!("var"),
    pub ident: Ident,
    pub ty: Option<VarType>,
    pub value: Option<VarValue>,
}
impl Peek for Var {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        <Keyword!("var")>::peek(input, errors, bound_to_line)
            || <Keyword!("mut")>::peek(input, errors, bound_to_line)
    }
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct VarType {
    pub sep: Punct!(":"),
    pub ty: Type,
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct VarValue {
    pub sep: Punct!("="),
    pub value: Expr,
}
