use oath_parser::{Endless, Parse};

use crate::ModSegment;

#[derive(Parse)]
pub struct ModContent {
    pub segments: Endless<ModSegment>,
}
