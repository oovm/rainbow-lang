use html_parser::{Dom, Node};

use crate::Result;

use crate::{Element, RainbowError, RenderNode};

impl TryFrom<Dom> for RenderNode {
    type Error = RainbowError;

    fn try_from(dom: Dom) -> Result<Self> {
        if !dom.errors.is_empty() {
            return Err(RainbowError::syntax_error(dom.errors[0].to_string()));
        }
        let mut root = vec![];
        for node in dom.children {
            root.push(RenderNode::try_from(node)?)
        }
        Ok(RenderNode::Root(root))
    }
}

impl TryFrom<Node> for RenderNode {
    type Error = RainbowError;

    fn try_from(node: Node) -> Result<Self> {
        match node {
            Node::Text(v) => Ok(RenderNode::Text(v)),
            Node::Element(v) => Ok(RenderNode::Element(Element::try_from(v)?)),
            Node::Comment(_) => Ok(RenderNode::Text(String::new())),
        }
    }
}

impl TryFrom<html_parser::Element> for Element {
    type Error = RainbowError;

    fn try_from(element: html_parser::Element) -> Result<Self> {
        Ok(Element {
            name: element.name.split(".").map(|s| s.to_string()).collect(),
            attributes: Default::default(),
            text: vec![],
        })
    }
}
