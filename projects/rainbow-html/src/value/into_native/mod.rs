use crate::{value::Text, Value};

impl From<Text> for String {
    fn from(v: Text) -> Self {
        v.value
    }
}

impl From<Value> for String {
    fn from(_: Value) -> Self {
        unimplemented!()
    }
}
