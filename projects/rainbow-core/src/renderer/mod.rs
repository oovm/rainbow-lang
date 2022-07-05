use std::collections::{BTreeMap, HashSet};

use crate::vm::RainbowVM;

mod methods;

pub struct RainbowRenderer<'vm> {
    vm: &'vm RainbowVM,
    theme: &'vm str,
    language: &'vm str,
    tracing: HashSet<String>,
}

#[derive(Clone, PartialEq)]
pub struct RenderFragment {
    inner: Vec<RenderNode>,
}

/// RenderNode
#[derive(Clone, PartialEq)]
pub struct RenderNode {
    pub name: Vec<String>,
    pub attributes: BTreeMap<String, String>,
    pub text: String,
}
