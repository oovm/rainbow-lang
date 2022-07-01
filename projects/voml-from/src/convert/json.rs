use crate::Value;
use indexmap::IndexMap;
use serde_json::Number;
use std::mem::transmute;

type Json = serde_json::Value;

impl From<Json> for Value {
    fn from(json: Json) -> Self {
        match json {
            Json::Null => Self::Null,
            Json::Bool(v) => v.into(),
            Json::Number(v) => v.into(),
            Json::String(v) => v.into(),
            Json::Array(v) => v.into(),
            Json::Object(v) => {
                let mut dict = IndexMap::new();
                for (k, v) in v.iter() {
                    dict.insert(String::from(k), Value::from(v.clone()));
                }
                dict.into()
            }
        }
    }
}

#[allow(dead_code)]
enum JsonNumber {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl From<Number> for Value {
    fn from(n: Number) -> Self {
        let inner = unsafe { transmute::<Number, JsonNumber>(n) };
        match inner {
            JsonNumber::PosInt(n) => n.into(),
            JsonNumber::NegInt(n) => n.into(),
            JsonNumber::Float(n) => n.into(),
        }
    }
}
