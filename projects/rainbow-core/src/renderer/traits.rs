use std::fmt::Formatter;

use super::*;

impl Debug for RenderNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderNode::Root(v) => f.debug_list().entries(v.iter()).finish(),
            RenderNode::Text(v) => {
                f.write_str("Text(")?;
                f.write_str(&v)?;
                f.write_str(")")
            }
            RenderNode::Element(v) => Debug::fmt(v, f),
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("name", &self.name.join("."))
            .field("attributes", &self.attributes)
            .field("children", &self.children)
            .finish()
    }
}
