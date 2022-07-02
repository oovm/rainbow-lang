use std::collections::BTreeMap;

use crate::Schema;

pub struct RainbowVM {
    schemas: BTreeMap<String, Schema>,
}
