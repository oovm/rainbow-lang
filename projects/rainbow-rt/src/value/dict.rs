use super::*;
use indexmap::map::Entry;
use std::ops::AddAssign;

/// ```note
/// { }
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct Dict {
    handler: Option<String>,
    value: IndexMap<String, Value>,
}

impl Debug for Dict {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}", s)?,
            None => (),
        }
        Debug::fmt(&self.value, f)
    }
}

impl Default for Dict {
    fn default() -> Self {
        Self { handler: None, value: IndexMap::new() }
    }
}

macro_rules! native2dict {
    ($T:ty) => {
    impl<K, V> From<$T> for Dict
    where
        K: Into<String>,
        V: Into<Value>,
    {
        fn from(input: $T) -> Self {
            let mut dict = IndexMap::new();
            for (k, v) in input.into_iter() {
                dict.insert(k.into(), v.into());
            }
            Self { handler: None, value: dict }
        }
    }
    };
    ($($T:ty), +) => {
        $(wrap_native!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2dict!($T);
    impl<K, V> From<$T> for Value
    where
        K: Into<String>,
        V: Into<Value>,
    {
        fn from(v: $T) -> Self {
            Self::Dict(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![IndexMap<K, V>, HashMap<K, V>, BTreeMap<K, V>];

impl From<Dict> for Value {
    fn from(v: Dict) -> Self {
        Value::Dict(Box::new(v))
    }
}

impl AddAssign for Dict {
    fn add_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.iter() {
            let mergeable = match self.get(&key) {
                None => false,
                Some(s) => value.is_dict() && s.is_dict(),
            };
            match mergeable {
                true => {
                    self.get_mut(&key).map(|e| e.merge(value.clone()));
                }
                false => {
                    self.insert(key.to_string(), value.clone());
                }
            }
        }
    }
}

impl Dict {
    ///
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.value.get(key)
    }
    ///
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.value.get_mut(key)
    }
    ///
    pub fn get_key(&self, key: &Text) -> Option<&Value> {
        self.value.get(&key.value)
    }
    ///
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
    ///
    pub fn ensure_key(&mut self, key: Text) -> &'_ mut Value {
        self.entry(key.value).or_default()
    }
}

impl Dict {
    ///
    pub fn empty() -> Value {
        Value::from(Dict::default())
    }
    ///
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    ///
    pub fn length(&self) -> usize {
        self.value.len()
    }
    ///
    pub fn iter(&self) -> indexmap::map::Iter<String, Value> {
        self.value.iter()
    }
    ///
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.value.insert(key, value)
    }
    ///
    pub fn as_vec(&self) -> Vec<Value> {
        self.value.values().cloned().collect()
    }
    ///
    pub fn entry(&mut self, key: String) -> Entry<String, Value> {
        self.value.entry(key)
    }
}
