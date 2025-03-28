use std::sync::Mutex;

mod diagnostics;
mod interner;
pub use diagnostics::*;
pub use interner::*;

#[derive(Debug)]
pub struct Context {
    diagnostics: Diagnostics,
    interner: Interner,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextHandle<'ctx>(pub &'ctx Mutex<Context>);

impl Context {
    pub fn new() -> Self {
        Context {
            diagnostics: Diagnostics::new(),
            interner: Interner::new(),
        }
    }
}
