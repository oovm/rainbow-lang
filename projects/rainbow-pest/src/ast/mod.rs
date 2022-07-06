use std::collections::HashMap;
use std::ops::Range;

use hex_color::HexColor;

mod object;
mod value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTProgram {
    pub statements: Vec<ASTStatement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTStatement {
    Schema(SchemaStatement),
    Meta(MetaStatement),
    Language(LanguageStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaStatement {
    pub schema: String,
    pub inherit: Vec<String>,
    pub object: RangedObject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaStatement {
    pub meta: String,
    pub object: RangedObject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageStatement {
    pub language: String,
    pub inherit: Vec<String>,
    pub attributes: RangedObject,
    pub range: Range<(u32, u32)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangedObject {
    pub inherit: Vec<String>,
    pub inner: HashMap<String, RangedValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KvPair {
    pub key: String,
    pub value: RangedValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RangedValue {
    Null,
    String(String),
    Number(String),
    Boolean(bool),
    Color(HexColor),
    Array(Vec<RangedValue>),
    Namespace(Vec<String>),
    Object(RangedObject),
}
