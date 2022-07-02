mod parser;

#[derive(Debug, Clone, PartialEq)]
pub struct RainbowError {
    kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {}

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {}

impl Default for Schema {
    fn default() -> Self {
        Self {}
    }
}

impl Schema {}
