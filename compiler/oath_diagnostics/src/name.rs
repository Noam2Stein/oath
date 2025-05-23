use super::*;

#[derive(Debug, Clone, Copy, Hash, Spanned, InternedDisplay)]
pub enum NameError {
    #[display("`{field_0}` doesn't exist in this context")]
    DoesntExist(Ident),
    #[display("`{field_0}` already exists in this context")]
    AlreadyExists(Ident),
}
