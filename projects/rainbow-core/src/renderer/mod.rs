use std::collections::{BTreeMap, HashMap, HashSet};

use crate::vm::RainbowVM;

#[cfg(feature = "html")]
mod from_html;

#[cfg(feature = "html")]
pub use self::from_html::parse_rainbow_html;

mod methods;

pub struct RainbowRenderer<'vm> {
    vm: &'vm RainbowVM,
    theme: &'vm str,
    language: &'vm str,
    tracing: HashSet<String>,
}

/// RenderNode
#[derive(Clone, PartialEq)]
pub struct RenderNode {
    pub name: Vec<String>,
    pub attributes: BTreeMap<String, Option<String>>,
    pub text: String,
}
