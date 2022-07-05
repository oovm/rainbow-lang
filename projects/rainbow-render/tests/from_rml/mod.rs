use std::convert::TryFrom;

use html_parser::Dom;

use rainbow_core::{parse_rainbow_html, RainbowRenderer, RainbowVM, RenderNode, Result};

#[test]
fn main() -> Result<()> {
    let html = include_str!("main.rml");
    let nodes = parse_rainbow_html(html)?;
    let vm = RainbowVM::builtin();
    let mut renderer = RainbowRenderer::new(&vm, "default", "rust");
    let out = renderer.render_html(&nodes)?;
    println!("{}", out);
    Ok(())
}
