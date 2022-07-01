use crate::Value;
use indexmap::map::IndexMap;

type Toml = toml::Value;

impl From<Toml> for Value {
    fn from(toml: Toml) -> Self {
        match toml {
            Toml::String(v) => v.into(),
            Toml::Integer(v) => v.into(),
            Toml::Float(v) => v.into(),
            Toml::Boolean(v) => v.into(),
            // FIXME: turn into utc
            Toml::Datetime(v) => format!("{}", v).into(),
            Toml::Array(v) => v.into(),
            Toml::Table(v) => {
                let mut dict = IndexMap::new();
                for (k, v) in v.iter() {
                    dict.insert(String::from(k), Value::from(v.clone()));
                }
                dict.into()
            }
        }
    }
}
