use std::collections::{BTreeMap, HashSet};

use crate::vm::RainbowVM;

mod iter;
mod methods;

pub struct RainbowRenderer<'vm> {
    pub vm: &'vm RainbowVM,
    pub theme: &'vm str,
    pub language: &'vm str,
    tracing: HashSet<String>,
    class_name: Option<String>,
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

pub fn html_escape(s: &str) -> String {
    s.replace("<", "&lt;").replace(">", "&gt;")
}
