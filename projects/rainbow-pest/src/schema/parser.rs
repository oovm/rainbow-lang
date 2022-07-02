use pest::error::Error;
use std::str::FromStr;

use crate::ast::ASTProgram;
use crate::{ParserConfig, Rule};

use super::*;

impl From<Error<Rule>> for RainbowError {
    fn from(e: Error<Rule>) -> Self {
        todo!()
    }
}

impl FromStr for Schema {
    type Err = RainbowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = ParserConfig::default();
        let ast = parser.parse(s)?;
        Ok(Schema::from(ast))
    }
}

impl From<ASTProgram> for Schema {
    fn from(program: ASTProgram) -> Self {
        let mut out = Schema::default();
        for i in program.statements {
            println!("{:?}", i);
        }
        todo!()
    }
}
