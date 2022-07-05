use std::collections::BTreeMap;

#[cfg(feature = "html")]
mod from_html;
mod traits;
use std::fmt::{Debug, Display};

/// RenderNode
#[derive(Clone, PartialEq)]
pub enum RenderNode {
    /// Root
    Root(Vec<RenderNode>),
    /// Text
    Text(String),
    /// Element
    Element(Element),
}

/// Element
#[derive(Clone, PartialEq)]
pub struct Element {
    pub name: Vec<String>,
    pub attributes: BTreeMap<String, Option<String>>,
    pub children: Vec<RenderNode>,
}
