use std::fmt::Write;

use rainbow_core::{RainbowRenderer, RenderFragment, RenderNode, Result};

pub trait RenderHtmlCodeSpan {
    fn render_html(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()>;
    fn render_css(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()>;
    fn render_scss(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()>;
}

impl RenderHtmlCodeSpan for RenderFragment {
    fn render_html(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<String> {
        ctx.clear_buffer();
        write!(ctx, "<code class=\"{}\">\n", ctx.get_class_name())?;
        for node in self {
            node.render_html(ctx)?;
        }
        write!(ctx, "</code>")?;
        todo!("{}", ctx.get_buffer())
    }

    fn render_css(&self, ctx: &mut RainbowRenderer) -> Result<String> {
        ctx.clear_buffer();
        todo!()
    }

    fn render_scss(&self, ctx: &mut RainbowRenderer) -> Result<String> {
        ctx.clear_buffer();
        todo!()
    }
}

impl RenderHtmlCodeSpan for RenderNode {
    fn render_html(&self, ctx: &mut RainbowRenderer) -> Result<String> {
        match self.name.is_empty() {
            true => return self.text.render_html(ctx),
            false => {

            }
        }

        let attrs = self.attributes.iter().map(|(k, v)| format!(" {}=\"{}\"", k, v)).collect::<String>();
        write!(
            ctx,
            "<{name}{attributes}>{text}</{name}>",
            name = self.name.join("."),
            text = html_escape(&self.text),
            attributes = attrs
        )
        Ok(())

    }

    fn render_css(&self, _: &mut RainbowRenderer) -> Result<String> {
        unreachable!()
    }

    fn render_scss(&self, _: &mut RainbowRenderer) -> Result<String> {
        unreachable!()
    }
}

impl RenderHtmlCodeSpan for String {
    fn render_html(&self, ctx: &mut RainbowRenderer) -> Result<String> {
        todo!()
    }

    fn render_css(&self, _: &mut RainbowRenderer) -> Result<String> {
        unreachable!()
    }

    fn render_scss(&self, _: &mut RainbowRenderer) -> Result<String> {
        unreachable!()
    }
}