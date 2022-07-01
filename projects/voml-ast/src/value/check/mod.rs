use super::*;
use num::Zero;

impl Value {
    ///
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
    ///
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }
    ///
    pub fn is_true(&self) -> bool {
        matches!(self, Value::Boolean(true))
    }
    ///
    pub fn is_false(&self) -> bool {
        matches!(self, Value::Boolean(false))
    }
    ///
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }
    ///
    pub fn is_dict(&self) -> bool {
        matches!(self, Value::Dict(_))
    }
    ///
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    ///
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Decimal(_))
    }
    ///
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }
    ///
    pub fn is_decimal(&self) -> bool {
        matches!(self, Value::Decimal(_))
    }
}

impl Value {
    ///
    pub fn is_empty(&self) -> bool {
        match self {
            Value::Dict(v) => v.is_empty(),
            Value::List(v) => v.is_empty(),
            Value::String(v) => v.is_empty(),
            _ => false,
        }
    }
    ///
    pub fn is_zero(&self) -> bool {
        match self {
            Value::Integer(n) => n.is_zero(),
            Value::Decimal(n) => n.is_zero(),
            _ => false,
        }
    }
}

impl Value {
    /// Note that a value of null and non-existent key are considered equivalent
    pub fn has_key(&self, key: &str) -> bool {
        match self {
            Value::Dict(dict) => dict.get(key).map(|e| !e.is_null()).unwrap_or_default(),
            _ => false,
        }
    }
    ///
    pub fn get_handler(&self) -> Option<String> {
        match self {
            Value::Null | Value::Boolean(_) | Value::Raw(_) => None,
            Value::Integer(v) => v.get_handler(),
            Value::Decimal(v) => v.get_handler(),
            Value::String(v) => v.get_handler(),
            Value::List(v) => v.get_handler(),
            Value::Dict(v) => v.get_handler(),
        }
    }
}
