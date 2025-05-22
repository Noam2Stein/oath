use super::*;

#[derive(Debug, Clone, Copy, Hash, Spanned, InternedDisplay)]
pub enum NameError {
    #[display("`{field_0}` doesn't exist in this context")]
    DoesntExist(Ident),
}
