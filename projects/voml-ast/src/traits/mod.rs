use crate::{Result, Value};

pub trait ExtendFormat {
    fn parse(&self, input: &str) -> Result<Value>;
}
