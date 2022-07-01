use super::*;
use std::{
    collections::btree_map::Entry,
    ops::{AddAssign, Neg},
};

///
#[derive(Clone, Eq, PartialEq)]
pub struct List {
    handler: Option<String>,
    value: BTreeMap<usize, Value>,
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}", s)?,
            None => (),
        }
        f.debug_list().entries(self.value.values()).finish()
    }
}

impl Default for List {
    fn default() -> Self {
        Self { handler: None, value: Default::default() }
    }
}

macro_rules! native2list {
    ($T:ty) => {
    impl<V> From<$T> for List
    where
        V: Into<Value>,
    {
        fn from(input: $T) -> Self {
            let mut list = BTreeMap::new();

            for (i, v) in input.into_iter().enumerate() {
                list.insert(i, v.into());
            }
            Self { handler: None, value: list }
        }
    }
    };
    ($($T:ty), +) => {
        $(native2list!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2list!($T);
    impl<V> From<$T> for Value
    where
        V: Into<Value>,
    {
        fn from(v: $T) -> Self {
            Self::List(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![Vec<V>, VecDeque<V>, LinkedList<V>, HashSet<V>, BTreeSet<V>];

impl From<List> for Value {
    fn from(v: List) -> Self {
        Value::List(Box::new(v))
    }
}

impl AddAssign<List> for List {
    fn add_assign(&mut self, rhs: List) {
        for (k, v) in rhs.value {
            self.value.insert(k, v);
        }
    }
}

// impl Index<usize> for List {
//     type Output = Value;
//     fn index(&self, n: usize) -> &Self::Output {
//         self.value.index(n)
//     }
// }

impl List {
    ///
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
    ///
    pub fn get_index(&self, index: &Integer) -> Option<&Value> {
        match index.get_index() {
            Some(i_index) => {
                let u_index = match i_index >= 0 {
                    true => i_index as usize,
                    false => 1 + self.length() - i_index.neg() as usize,
                };
                self.value.get(&u_index)
            }
            None => None,
        }
    }
    ///
    pub fn ensure_index(&mut self, index: Integer) -> &'_ mut Value {
        match index.get_index() {
            Some(i_index) => {
                let u_index = match i_index >= 0 {
                    true => i_index as usize,
                    false => 1 + self.length() - i_index.neg() as usize,
                };
                self.entry(u_index).or_default()
            }
            None => unreachable!(),
        }
    }
}

impl List {
    ///
    pub fn empty() -> Value {
        Value::from(List::default())
    }
    ///
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    ///
    pub fn length(&self) -> usize {
        // match self.value.last_key_value() {
        //     Some((n, _)) => {*n},
        //     None => {0}
        // }
        match self.value.keys().rev().next() {
            Some(n) => *n,
            None => 0,
        }
    }
    ///
    pub fn as_vec(&self) -> Vec<Value> {
        self.value.values().cloned().collect()
    }
    ///
    pub fn entry(&mut self, index: usize) -> Entry<'_, usize, Value> {
        self.value.entry(index)
    }
    ///
    pub fn get(&self, index: &str) -> Option<&Value> {
        let i = match isize::from_str(index) {
            Ok(o) => o,
            Err(_) => return None,
        };
        match i > 0 {
            true => self.value.get(&(i as usize)),
            false => self.value.get(&((self.value.len() as isize + i) as usize)),
        }
    }
    ///
    pub fn extend(&mut self, item: impl Into<List>) {
        self.value.extend(item.into().value)
    }
}
