use std::collections::HashMap;

use oath_context::StrId;

use crate::NameId;

pub type Namespace = HashMap<StrId, NameId>;
