use rainbow_pest::HexColor;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};
mod methods;
mod parser;
mod value;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Schema {
    pub schema: String,
    pub theme: String,
    pub variant: String,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Number(String),
    Boolean(bool),
    Color(HexColor),
    Array(Vec<Value>),
    Namespace(Vec<String>),
    Object(HashMap<String, Value>),
}
