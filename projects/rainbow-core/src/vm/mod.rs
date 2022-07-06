use std::collections::BTreeMap;

use crate::{schema::Value, Schema};

mod builtin;
mod parser;
mod reference;

pub struct RainbowVM {
    schemas: BTreeMap<String, Schema>,
}

impl RainbowVM {
    pub fn builtin() -> Self {
        Self { schemas: BTreeMap::new() }
    }
}
