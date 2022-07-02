use super::*;

impl RangedObject {
    #[inline]
    pub fn new() -> Self {
        Self { inherit: vec![], inner: HashMap::new() }
    }

    #[inline]
    pub fn insert(&mut self, key: String, value: RangedValue) {
        self.inner.insert(key, value);
    }
    #[inline]
    pub fn insert_pair(&mut self, pair: KvPair) {
        self.inner.insert(pair.key, pair.value);
    }
    #[inline]
    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.inner.get(key).cloned() {
            Some(RangedValue::String(s)) => Some(s),
            _ => None,
        }
    }
}

impl KvPair {
    #[inline]
    pub fn new(key: String, value: RangedValue) -> Self {
        Self { key, value }
    }
}
