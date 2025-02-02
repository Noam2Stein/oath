use oath_parser::{InAngles, Parse, Peek, SepEndless};
use oath_tokenizer::{keyword, punct, Ident};

#[derive(Debug, Clone, Parse, Peek)]
pub struct GenericParams(pub InAngles<SepEndless<GenericParam, punct!(",")>>);

#[derive(Debug, Clone, Parse)]
pub struct GenericParam {
    pub ident: Ident,
    pub content: GenericParamContent,
}

#[derive(Debug, Clone, Parse)]
pub enum GenericParamContent {
    Value(GenericValueParam),
    Type(GenericTypeParam),
}

#[derive(Debug, Clone, Parse)]
pub struct GenericTypeParam {}

#[derive(Debug, Clone, Parse, Peek)]
pub struct GenericValueParam {
    pub val_keyword: keyword!("val"),
}
