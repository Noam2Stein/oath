use oath_parser::{InAngles, Parse, Peek, SepEndless};
use oath_tokenizer::punct;

use crate::Expr;

#[derive(Parse, Peek)]
pub struct GenericArgs(pub InAngles<SepEndless<Expr, punct!(",")>>);
