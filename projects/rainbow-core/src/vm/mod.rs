use std::collections::BTreeMap;

use crate::{schema::Value, Schema};

mod builtin;
mod parser;
mod reference;

pub struct RainbowVM {
    schemas: BTreeMap<String, Schema>,
}
