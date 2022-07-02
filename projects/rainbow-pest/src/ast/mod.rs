use std::collections::HashMap;

use hex_color::HexColor;

mod object;
mod value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTProgram {
    pub statements: Vec<ASTStatement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTStatement {
    Import(ImportStatement),
    Schema(SchemaStatement),
    Meta(MetaStatement),
    Global(LanguageStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaStatement {
    pub schema: String,
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
    pub inherit: Option<String>,
    pub attributes: RangedObject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangedObject {
    pub inherit: Option<Namespace>,
    pub inner: HashMap<String, RangedValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KvPair {
    pub key: String,
    pub value: RangedValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Namespace {}

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
