use std::fmt::{Debug, Display, Formatter, Write};

use crate::{renderer::RainbowRenderer, vm::RainbowVM, Element, RenderNode, Result};

mod traits;

impl<'vm> RainbowRenderer<'vm> {
    pub fn new(vm: &'vm RainbowVM, theme: &'vm str, language: &'vm str) -> Self {
        Self { vm, theme, language }
    }
    pub fn render(&self, node: &RenderNode) -> Result<String> {
        let mut out = String::with_capacity(1024);
        node.write_rendered(self, &mut out)?;
        Ok(out)
    }
}

impl RenderNode {
    fn write_rendered<'vm>(&self, vm: &RainbowRenderer<'vm>, out: &mut String) -> Result<()> {
        match self {
            RenderNode::Root(v) => {
                for node in v {
                    node.write_rendered(vm, out)?;
                }
            }
            RenderNode::Text(v) => {
                for line in v.lines() {
                    writeln!(out, "<span>{}</span>", line)?;
                }
            }
            RenderNode::Element(v) => v.write_rendered(vm, out)?,
        }
        return Ok(());
    }
}

impl Element {
    fn write_rendered<'vm>(&self, vm: &RainbowRenderer<'vm>, out: &mut String) -> Result<()> {
        for line in self.text {
            line.write_rendered(vm, out)?;
        }

        return Ok(());
    }
}
