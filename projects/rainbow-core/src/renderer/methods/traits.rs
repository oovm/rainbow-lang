use std::fmt::{Arguments, Display, Write};

use super::*;

impl Debug for RenderFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Display for RenderFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in self.inner.iter() {
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

impl Debug for RenderNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("name", &self.name.join("."))
            .field("attributes", &self.attributes)
            .field("text", &self.text)
            .finish()
    }
}

impl Display for RenderNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.name.is_empty() {
            true => write!(f, "{}", self.text),
            false => {
                let attrs = self.attributes.iter().map(|(k, v)| format!(" {}=\"{}\"", k, v)).collect::<String>();
                write!(
                    f,
                    "<{name}{attributes}>{text}</{name}>",
                    name = self.name.join("."),
                    text = html_escape(&self.text),
                    attributes = attrs
                )
            }
        }
    }
}

impl Default for RenderFragment {
    fn default() -> Self {
        Self { inner: vec![] }
    }
}

impl<'w> Write for RainbowRenderer<'w> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}

fn html_escape(s: &str) -> String {
    s.replace("<", "&lt;").replace(">", "&gt;")
}
