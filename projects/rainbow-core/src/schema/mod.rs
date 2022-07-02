use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

use rainbow_pest::HexColor;

mod methods;
mod parser;
mod value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub schema: String,
    pub theme: String,
    pub variant: String,
    pub custom: BTreeMap<String, Value>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Reference(Vec<String>),
    Owned(Box<OwnedValue>),
}

#[derive(Clone, Eq, PartialEq)]
pub enum OwnedValue {
    Null,
    String(String),
    Number(String),
    Boolean(bool),
    Color(HexColor),
    Reference(Vec<String>),
    Object(BTreeMap<String, Value>),
}
