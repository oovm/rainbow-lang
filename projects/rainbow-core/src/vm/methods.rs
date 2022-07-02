use super::*;
use crate::schema::Value;

impl Default for RainbowVM {
    fn default() -> Self {
        Self { schemas: Default::default() }
    }
}

impl RainbowVM {
    pub fn resolve(&self, path: &[String]) -> Value {
        let mut path = path.iter().rev();
        match path.next() {
            None => {
                return Value::Null;
            }
            Some(s) => {}
        }

        let pointer = &mut self.schemas;
    }
}
