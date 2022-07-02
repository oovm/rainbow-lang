use super::*;

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => f.write_str("null"),
            Value::String(s) => f.write_str(s),
            Value::Number(_) => {
                todo!()
            }
            Value::Boolean(_) => {
                todo!()
            }
            Value::Color(c) => write!(f, "{}", c),
            Value::Reference(_) => {
                todo!()
            }
            Value::Object(o) => f.debug_map().entries(o.iter()).finish(),
        }
    }
}
