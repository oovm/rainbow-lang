
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTProgram {
    statements: Vec<ASTStatement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTStatement {
    Import(ImportStatement),
    Schema(SchemaStatement),
    Meta(MetaStatement),
    Global(LanguageStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportStatement {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaStatement {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaStatement {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageStatement {

}