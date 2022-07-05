use scraper::{node::Text, ElementRef, Html, Selector};

use rainbow_core::{RenderFragment, Result};

#[test]
fn find_highlight_js() -> Result<()> {
    let fragment = Html::parse_fragment(include_str!("hljs.html"));
    let selector = Selector::parse("code.hljs").unwrap();

    for element in fragment.select(&selector) {
        println!("{:#?}", element);
        refine_highlight_js(element)?;
        todo!()
    }
    Ok(())
}

fn refine_highlight_js(element: ElementRef) -> Result<RenderFragment> {
    let mut out = RenderFragment::default();
    for node in element.children() {
        if let Some(s) = node.value().as_text() {
            out.insert_text(s.to_string());
            continue;
        }
        if let Some(s) = node.value().as_element() {
            println!("{:#?}", s);
            continue;
        }
    }
    todo!()
}
