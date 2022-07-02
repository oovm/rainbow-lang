#[derive(Debug, Clone, PartialEq)]
pub struct RainbowError {
    kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    DuplicateDeclaration(String),
}

impl RainbowError {
    pub fn duplicate_declaration(name: &str) -> Self {
        Self { kind: ErrorKind::DuplicateDeclaration(name.to_string()) }
    }
}
