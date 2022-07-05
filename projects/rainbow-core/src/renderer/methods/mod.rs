use std::fmt::{Debug, Formatter, Write};

use crate::{renderer::RainbowRenderer, vm::RainbowVM, RenderNode, Result};

mod traits;

impl<'vm> RainbowRenderer<'vm> {
    pub fn new(vm: &'vm RainbowVM, theme: &'vm str, language: &'vm str) -> Self {
        Self { vm, theme, language, tracing: Default::default() }
    }
    pub fn render_html(&mut self, nodes: &[RenderNode]) -> Result<String> {
        let mut out = String::with_capacity(1024);
        out.write_str("<code>\n")?;
        for node in nodes {
            node.write_rendered(self, &mut out)?;
        }
        out.write_str("</code>")?;
        Ok(out)
    }
    pub fn clear_tracing(&mut self) {
        self.tracing.clear();
    }
    pub fn render_css(&self) -> String {
        format!("{:#?}", self.tracing)
    }

    pub fn trace(&mut self, name: &[String]) {
        self.tracing.insert(name.join("-"));
    }
}

impl RenderNode {
    pub fn text<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: vec![], attributes: Default::default(), text: s.into() }
    }

    fn write_rendered<'vm>(&self, vm: &mut RainbowRenderer<'vm>, out: &mut String) -> Result<()> {
        for line in self.text.lines() {
            if self.name.is_empty() {
                writeln!(out, "<span>{text}</span>", text = line)?;
            }
            else {
                vm.trace(&self.name);
                writeln!(out, "<span class='{class}'>{text}</span>", class = self.name.join("-"), text = line)?;
            }
        }
        return Ok(());
    }
}
