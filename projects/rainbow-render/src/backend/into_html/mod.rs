use std::{collections::BTreeMap, fmt::Write};

use rainbow_core::{html_escape, RainbowRenderer, RainbowVM, RenderFragment, RenderNode, Result};

mod display;

pub struct HtmlInlineRenderer<'vm> {
    vm: &'vm RainbowVM,
    buffer: String,
    class_name: String,
}

impl<'vm> HtmlInlineRenderer<'vm> {
    pub fn new(vm: &'vm RainbowVM) -> Self {
        Self { vm, buffer: "".to_string(), class_name: "rainbow".to_string() }
    }
    pub fn set_class_name<S>(&mut self, class_name: S)
    where
        S: ToString,
    {
        self.class_name = class_name.to_string();
    }
}

impl<'vm> HtmlInlineRenderer<'vm> {
    pub fn render(&mut self, code: &RenderFragment) -> Result<String> {
        self.buffer.clear();
        self.render_fragment(code)?;
        Ok(std::mem::take(&mut self.buffer))
    }
    fn render_fragment(&mut self, code: &RenderFragment) -> Result<()> {
        write!(self.buffer, "<pre><code class=\"{}\">", self.class_name)?;
        for node in code {
            self.render_node(node)?;
        }
        write!(self, "</code></pre>")?;
        Ok(())
    }
    fn render_node(&mut self, node: &RenderNode) -> Result<()> {
        if let true = node.name.is_empty() {
            return self.render_text(&node.text);
        }
        let mut object = BTreeMap::default();
        let mut tag = "span";
        if let Some(link) = node.attributes.get("link") {
            tag = "a";
            object.insert("href", link);
        }
        let value = self.vm.try_reference(&node.name);
        write!(self, "<{tag}>{text}</{tag}>", tag = tag, text = html_escape(&node.text))?;
        Ok(())
    }
    fn render_text(&mut self, text: &str) -> Result<()> {
        for c in text.chars() {
            match c {
                '\n' => self.write_str("<br>")?,
                '<' => self.write_str("&lt;")?,
                '>' => self.write_str("&gt;")?,
                _ => self.write_char(c)?,
            }
        }
        Ok(())
    }
}
