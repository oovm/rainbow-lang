use std::collections::{BTreeMap, HashMap};

/// RenderNode
#[derive(Debug, Clone, PartialEq)]
pub enum RenderNode {
    /// Text
    Text(String),
    /// Element
    Element(Element),
}

/// Element
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// The name / tag of the element
    pub name: String,
    /// All of the elements attributes, except id and class
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub attributes: BTreeMap<String, Option<String>>,
    /// All of the elements child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<RenderNode>,
}
