use crate::Result;

use super::*;

impl Default for RainbowVM {
    fn default() -> Self {
        Self { schemas: Default::default() }
    }
}

impl RainbowVM {
    pub fn builtin(&self) -> Self {
        let mut vm = Self::default();
        vm.define_schema(include_str!("one-dark.rmb")).unwrap();
        vm.define_schema(include_str!("one-light.rmb")).unwrap();
        vm.define_schema(include_str!("neo-night.rmb")).unwrap();
        vm.define_schema(include_str!("neo-light.rmb")).unwrap();
        return vm;
    }
    pub fn define_schema(&mut self, text: &str) -> Result<()> {
        todo!()
    }
}
