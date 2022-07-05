use std::collections::BTreeMap;

use crate::vm::RainbowVM;

#[cfg(feature = "html")]
mod from_html;
mod methods;

pub struct RainbowRenderer<'vm> {
    vm: &'vm RainbowVM,
    theme: &'vm str,
    language: &'vm str,
}

/// RenderNode
#[derive(Clone, PartialEq)]
pub struct RenderNode {
    pub name: Vec<String>,
    pub attributes: BTreeMap<String, Option<String>>,
    pub text: String,
}
