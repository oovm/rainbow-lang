use html_parser::Dom;
use rainbow_core::{RenderNode, Result};
use std::convert::TryFrom;

#[test]
fn main() -> Result<()> {
    let html = include_str!("main.rml");
    let dom = Dom::parse(html)?;
    println!("{:#?}", RenderNode::try_from(dom)?);
    Ok(())
}
