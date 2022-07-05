use html_parser::{Dom, Element, Node};

use crate::{RainbowError, RenderNode, Result};

pub fn parse_rainbow_html(html: &str) -> Result<Vec<RenderNode>> {
    let dom = Dom::parse(html)?;
    if !dom.errors.is_empty() {
        return Err(RainbowError::syntax_error(dom.errors[0].to_string()));
    }
    let mut root = vec![];
    for node in dom.children {
        root.push(RenderNode::try_from(node)?)
    }
    Ok(root)
}

impl TryFrom<Node> for RenderNode {
    type Error = RainbowError;

    fn try_from(node: Node) -> Result<Self> {
        match node {
            Node::Text(v) => Ok(RenderNode::text(v)),
            Node::Element(v) => Ok(RenderNode::try_from(v)?),
            Node::Comment(_) => Ok(RenderNode::text("")),
        }
    }
}

impl TryFrom<Element> for RenderNode {
    type Error = RainbowError;

    fn try_from(element: Element) -> Result<Self> {
        let mut text = String::new();
        for node in element.children {
            match node {
                Node::Text(s) => text = s,
                Node::Element(_) => return Err(RainbowError::syntax_error("Nested elements are not supported".to_string())),
                Node::Comment(_) => {}
            }
        }
        Ok(RenderNode {
            name: element.name.split(".").map(|s| s.to_string()).collect(),
            attributes: element.attributes.into_iter().collect(),
            text,
        })
    }
}
