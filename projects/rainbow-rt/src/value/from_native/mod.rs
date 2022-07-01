mod from_tuples;
mod parse;

pub use parse::parse_number;

use crate::Value;
use bigdecimal::BigDecimal;
use num::{BigInt, Num};
use std::str::FromStr;

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl<O, E> From<Result<O, E>> for Value
where
    O: Into<Value>,
{
    fn from(value: Result<O, E>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(_) => Value::Null,
        }
    }
}
