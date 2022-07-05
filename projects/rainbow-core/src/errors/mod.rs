#[cfg(feature = "html")]
mod from_html;
pub type Result<T> = std::result::Result<T, RainbowError>;

#[derive(Debug, Clone, PartialEq)]
pub struct RainbowError {
    kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    DuplicateDeclaration(String),
    SyntaxError(String),
}

impl RainbowError {
    pub fn duplicate_declaration(name: &str) -> Self {
        Self { kind: ErrorKind::DuplicateDeclaration(name.to_string()) }
    }
    pub fn syntax_error<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        Self { kind: ErrorKind::SyntaxError(msg.into()) }
    }
}
