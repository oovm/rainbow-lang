use super::*;
use crate::schema::Value;

impl Default for RainbowVM {
    fn default() -> Self {
        Self { schemas: Default::default() }
    }
}

impl RainbowVM {
    pub fn resolve(&self, path: &[String]) -> Value {
        self.resolve_optional(path).unwrap_or_else(|| Value::null())
    }
    pub(crate) fn resolve_optional(&self, path: &[String]) -> Option<Value> {
        let mut path = path.iter().rev();
        let schema = self.schemas.get(path.next()?)?;
        todo!()
    }
}
