use crate::Value;
use indexmap::map::IndexMap;
use yaml_rust::{yaml::Hash, Yaml};

pub trait ToArc {
    fn to_arc(&self) -> String;
}

impl From<Yaml> for Value {
    fn from(yaml: Yaml) -> Self {
        match yaml {
            Yaml::Null => Value::Null,
            Yaml::BadValue => Value::Null,
            Yaml::Integer(v) => v.into(),
            Yaml::Real(r) => {
                if r.to_lowercase().contains("n") {
                    // match r.to_lowercase().as_str() {
                    //     ".inf" | "+.inf" | "inf" => format!("{:#X}f64", f64::INFINITY.to_bits()),
                    //     "-inf" | "-.inf" => format!("{:#X}f64", f64::NEG_INFINITY.to_bits()),
                    //     ".nan" | "nan" => format!("{:#X}f64", f64::NAN.to_bits()),
                    //     _ => Value::Null,
                    // }
                    return Value::Null;
                }
                let r = if r.starts_with('.') {
                    format!("0{}", r)
                }
                else if r.ends_with('.') {
                    format!("{}0", r)
                }
                else {
                    r
                };
                match r.parse::<f64>() {
                    Ok(o) => o.into(),
                    Err(_) => Value::Null,
                }
            }
            Yaml::String(v) => v.into(),
            Yaml::Boolean(v) => v.into(),
            Yaml::Array(v) => v.into(),
            Yaml::Hash(v) => v.into(),
            Yaml::Alias(a) => {
                println!("{:#?}", a);
                unreachable!()
            }
        }
    }
}

impl From<Hash> for Value {
    fn from(v: Hash) -> Self {
        let mut dict = IndexMap::new();
        for (k, v) in v.iter() {
            let k = match k {
                Yaml::Null => String::from("null"),
                Yaml::String(s) => s.to_owned(),
                _ => unimplemented!("{:?}", k),
            };
            dict.insert(k, Value::from(v.clone()));
        }
        dict.into()
    }
}
