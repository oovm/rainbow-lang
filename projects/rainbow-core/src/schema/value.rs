use std::fmt::Display;

use super::*;

impl Display for OwnedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for OwnedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnedValue::Null => f.write_str("null"),
            OwnedValue::String(s) => f.write_str(s),
            OwnedValue::Number(s) => f.write_str(s),
            OwnedValue::Boolean(s) => f.write_str(&s.to_string()),
            OwnedValue::Color(c) => write!(f, "{}", c),
            OwnedValue::Object(o) => {
                let debug = &mut f.debug_struct("Object");
                for (k, v) in o.iter() {
                    debug.field(k, v);
                }
                debug.finish()
            }
        }
    }
}

impl From<Schema> for Value {
    fn from(schema: Schema) -> Self {
        let mut out = BTreeMap::new();
        out.insert("theme".to_string(), Value::string(schema.theme));
        out.insert("variant".to_string(), Value::string(schema.variant));
        return Value::object(out);
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Reference(v) => f.write_str(&v.join(".")),
            Value::Owned(v) => f.write_str(&v.to_string()),
        }
    }
}

impl Value {
    pub fn null() -> Self {
        Value::Owned(Box::new(OwnedValue::Null))
    }
    pub fn string(s: String) -> Self {
        Value::Owned(Box::new(OwnedValue::String(s)))
    }
    pub fn boolean(s: bool) -> Self {
        Value::Owned(Box::new(OwnedValue::Boolean(s)))
    }
    pub fn number(s: String) -> Self {
        Value::Owned(Box::new(OwnedValue::Number(s)))
    }
    pub fn color(s: HexColor) -> Self {
        Value::Owned(Box::new(OwnedValue::Color(s)))
    }
    pub fn object(s: BTreeMap<String, Value>) -> Self {
        Value::Owned(Box::new(OwnedValue::Object(s)))
    }
    pub fn reference(s: Vec<String>) -> Self {
        Value::Reference(s)
    }
}
