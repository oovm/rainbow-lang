use crate::{RainbowError, Result};
use rainbow_pest::{
    ast::{ASTProgram, ASTStatement, SchemaStatement},
    ParserConfig,
};
use std::str::FromStr;

use super::*;

impl Default for RainbowVM {
    fn default() -> Self {
        Self { schemas: Default::default() }
    }
}

impl RainbowVM {
    pub fn builtin() -> Self {
        let mut vm = RainbowVM::default();
        vm.define_schema(include_str!("one-dark.rmb")).unwrap();
        vm.define_schema(include_str!("one-light.rmb")).unwrap();
        vm.define_schema(include_str!("neo-night.rmb")).unwrap();
        vm.define_schema(include_str!("neo-light.rmb")).unwrap();
        return vm;
    }
}
