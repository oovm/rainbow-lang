use std::convert::TryFrom;

use html_parser::Dom;

use rainbow_core::{RainbowRenderer, RainbowVM, RenderNode, Result};

#[test]
fn main() -> Result<()> {
    let html = include_str!("main.rml");
    let dom = Dom::parse(html)?;
    let vm = RainbowVM::builtin();
    let renderer = RainbowRenderer::new(&vm, "default", "rust");
    let dom = RenderNode::try_from(dom)?;
    let out = renderer.render(&dom)?;
    println!("{}", out);
    Ok(())
}
