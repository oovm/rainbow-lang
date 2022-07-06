use std::{collections::BTreeMap, fmt::Write};

use rainbow_core::{html_escape, RainbowRenderer, RainbowVM, RenderFragment, RenderNode, Result};

mod display;

pub struct HtmlInlineRenderer<'vm> {
    vm: &'vm RainbowVM,
    theme: &'vm str,
    language: &'vm str,
    buffer: String,
    class_name: Option<String>,
}

impl<'vm> HtmlInlineRenderer<'vm> {
    pub fn new(vm: &'vm RainbowVM, theme: &'vm str, language: &'vm str) -> Self {
        Self { vm, theme, language, buffer: String::new(), class_name: None }
    }

    pub fn set_class_name(&mut self, class_name: &str) {
        self.class_name = Some(class_name.to_string());
    }
}

impl<'vm> HtmlInlineRenderer<'vm> {
    pub fn render_html(&mut self, code: &RenderFragment) -> Result<String> {
        self.buffer.clear();
        self.render_fragment(code)?;
        Ok(std::mem::take(&mut self.buffer))
    }
    fn render_fragment(&mut self, code: &RenderFragment) -> Result<()> {
        match &self.class_name {
            None => write!(self, "<pre><code>")?,
            Some(s) => write!(self, "<pre><code class=\"{}\">", s)?,
        }
        for node in self {
            node.render_html(ctx, buffer)?;
        }
        write!(self, "</code></pre>")?;
        Ok(())
    }
    fn render_node(&mut self, code: &RenderNode) -> Result<()> {
        if let true = self.name.is_empty() {
            return render_text(&self.text, buffer);
        }
        let mut object = BTreeMap::default();
        let mut tag = "span";
        if let Some(link) = self.attributes.get("link") {
            tag = "a";
            object.insert("href", link);
        }
        let value = self.vm.try_reference(&self.name);
        println!("{:?}", value);
        write!(self, "<{tag}>{text}</{tag}>", tag = tag, text = html_escape(&self.text))?;
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
