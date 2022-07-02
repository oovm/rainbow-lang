use super::*;

impl Object {
    #[inline]
    pub fn new() -> Self {
        Self { inherit: None, inner: HashMap::new() }
    }
    #[inline]
    pub fn insert(&mut self, key: String, value: RangedValue) {
        self.inner.insert(key, value);
    }
    #[inline]
    pub fn insert_pair(&mut self, pair: KvPair) {
        self.inner.insert(pair.key, pair.value);
    }
}

impl KvPair {
    #[inline]
    pub fn new(key: String, value: RangedValue) -> Self {
        Self { key, value }
    }
}
