use std::collections::BTreeMap;

use crate::Schema;

mod methods;
mod parser;

pub struct RainbowVM {
    schemas: BTreeMap<String, Schema>,
}

impl RainbowVM {
    pub fn builtin() -> Self {
        Self { schemas: BTreeMap::new() }
    }
}
