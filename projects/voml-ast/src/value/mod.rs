//! value types

mod access;
mod bytes;
mod check;
mod decimal;
mod dict;
mod from_native;
mod into_ast;
mod into_native;
mod list;
mod string;

pub use decimal::Decimal;
pub use dict::Dict;
pub use from_native::parse_number;
pub use integer::Integer;
pub use list::List;
pub use string::{Text, TextDelimiter};

use bigdecimal::BigDecimal;
use indexmap::IndexMap;
use num::{BigInt, BigUint};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
    str::FromStr,
};

/// All possible data types
#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    /// ```note
    /// null
    /// ```
    Null,
    /// ```note
    /// true | false
    /// ```
    Boolean(bool),
    /// ```note
    /// 0
    /// 123
    /// ```
    Integer(Box<Integer>),
    /// ```note
    /// 0.0
    /// 0.123
    /// ```
    Decimal(Box<Decimal>),
    /// ```note
    /// "s"
    /// ```
    String(Box<Text>),
    /// ```note
    /// [ ]
    /// [null, ture, false]
    /// ```
    List(Box<List>),
    /// ```note
    /// {}
    /// {a: null}
    /// ```
    Dict(Box<Dict>),
    /// ## Raw Data
    /// For FFI data exchange
    /// This type cannot be input.
    Raw(Box<[u8]>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(v) => Display::fmt(v, f),
            Value::Integer(v) => Display::fmt(v, f),
            Value::Decimal(v) => Display::fmt(v, f),
            Value::String(v) => Display::fmt(v, f),
            Value::List(v) => Debug::fmt(v, f),
            Value::Dict(v) => Debug::fmt(v, f),
            Value::Raw(v) => Debug::fmt(v, f),
        }
    }
}
