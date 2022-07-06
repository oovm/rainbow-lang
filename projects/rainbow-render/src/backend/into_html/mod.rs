use std::{collections::BTreeMap, fmt::Write};

use rainbow_core::{RainbowRenderer, RenderFragment, RenderNode, Result};

pub trait RenderHtmlCodeSpan {
    fn render_html(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()>;
    fn render_css(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()>;
    fn render_scss(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()>;
}

impl RenderHtmlCodeSpan for RenderFragment {
    fn render_html(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()> {
        write!(buffer, "<code class=\"{}\">\n", ctx.get_class_name())?;
        for node in self {
            node.render_html(ctx, buffer)?;
        }
        write!(buffer, "</code>")?;
        Ok(())
    }

    fn render_css(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()> {
        todo!()
    }

    fn render_scss(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()> {
        todo!()
    }
}

impl RenderHtmlCodeSpan for RenderNode {
    fn render_html(&self, ctx: &mut RainbowRenderer, buffer: &mut impl Write) -> Result<()> {
        if let true = self.name.is_empty() {
            return render_text(&self.text, buffer);
        }
        let mut object = BTreeMap::default();
        let mut tag = "span";
        if let Some(link) = self.attributes.get("link") {
            tag = "a";
            object.insert("href", link);
        }
        let value = ctx.vm.try_reference(&self.name);
        println!("{:?}", value);
        write!(buffer, "<{tag}>{text}</{tag}>", tag = tag, text = self.text)?;
        Ok(())
    }

    fn render_css(&self, _: &mut RainbowRenderer, _: &mut impl Write) -> Result<()> {
        todo!()
    }

    fn render_scss(&self, _: &mut RainbowRenderer, _: &mut impl Write) -> Result<()> {
        todo!()
    }
}

fn render_text(text: &str, buffer: &mut impl Write) -> Result<()> {
    if text.is_empty() || text.eq("\n") {
        return Ok(());
    }
    write!(buffer, "<span>{text}</span>", text = text)?;
    Ok(())
}

fn render_link(node: &RenderNode, buffer: &mut impl Write, link: &str) -> Result<()> {
    for line in node.text.lines() {
        write!(buffer, "<a href={}>{}</a>", link, line)?;
    }
    Ok(())
}
