use std::collections::HashMap;

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
pub struct SchemaStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaStatement {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageStatement {
    pub language: String,
    pub inherit: Option<String>,
    pub attributes: Object,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Object {
    inherit: Option<Namespace>,
    inner: HashMap<String, RangedValue>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Namespace {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RangedValue {
    Null,
    String(String),
    Number(String),
    Boolean(String),
    Object(Object),
}