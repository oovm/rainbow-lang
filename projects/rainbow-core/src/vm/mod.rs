use crate::Schema;
use std::collections::BTreeMap;

mod methods;
mod parser;

pub struct RainbowVM {
    schemas: BTreeMap<String, Schema>,
}
