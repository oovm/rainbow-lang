use super::*;

pub fn parse_rainbow_html(html: &str) -> Result<Vec<RenderNode>> {
    let dom = parse_html(html)?;
    let mut root = vec![];
    for node in dom.children {
        parse_node(&node, &mut root)?
    }
    Ok(root)
}

fn parse_node(node: &Node, root: &mut Vec<RenderNode>) -> Result<()> {
    match node {
        Node::Text(v) => {
            for line in v.lines() {
                root.push(RenderNode::text(line.to_string()));
            }
        }
        Node::Element(v) => parse_element(v, root)?,
        Node::Comment(_) => {}
    }
    Ok(())
}

pub fn parse_element(element: &Element, root: &mut Vec<RenderNode>) -> Result<()> {
    let name: Vec<String> = element.name.split(".").map(|s| s.to_string()).collect();
    let mut attributes = BTreeMap::<String, String>::default();
    for (k, v) in &element.attributes {
        attributes.insert(k.to_string(), v.clone().unwrap_or_default());
    }
    for node in &element.children {
        match node {
            Node::Text(text) => {
                for line in text.lines() {
                    root.push(RenderNode { name: name.clone(), attributes: attributes.clone(), text: line.to_string() });
                }
            }
            Node::Element(_) => return Err(RainbowError::syntax_error("Nested elements are not supported".to_string())),
            Node::Comment(_) => {}
        }
    }
    Ok(())
}
