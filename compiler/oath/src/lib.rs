use std::{collections::HashMap, sync::Arc};

use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_src::*;
use oath_tokenizer::*;

pub use oath_highlighting::HighlightColor;
pub use oath_src::{Position, Span};
pub use oath_tokens::KEYWORDS;

mod lib_;
mod mod_;
pub use lib_::*;
pub use mod_::*;

pub struct Oath {
    interner: Arc<Interner>,
}

impl Oath {
    pub fn new() -> Self {
        Self {
            interner: Arc::new(Interner::new()),
        }
    }

    pub fn create_lib(&self, mods: HashMap<ModPath, impl AsRef<str>>) -> Lib {
        Lib::new(self, mods)
    }
}
