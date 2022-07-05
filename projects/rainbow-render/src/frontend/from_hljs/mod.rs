use scraper::{
    node::{Element, Text},
    ElementRef, Html, Selector,
};
use selectors::attr::CaseSensitivity::AsciiCaseInsensitive;

use rainbow_core::{RenderFragment, RenderNode, Result};

#[test]

fn test_hljs() {
    let out = find_highlight_js(include_str!("hljs.html")).unwrap();
    for node in out {
        println!("{}", node);
    }
}

fn find_highlight_js(html: &str) -> Result<Vec<RenderFragment>> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("code.hljs").unwrap();
    let mut out = vec![];
    for element in fragment.select(&selector) {
        out.push(refine_code(element)?)
    }
    Ok(out)
}

fn refine_code(code: ElementRef) -> Result<RenderFragment> {
    let language = code
        .value()
        .classes
        .iter()
        .filter(|s| s.starts_with("language"))
        .map(|s| s.strip_prefix("language-").unwrap_or("").to_string())
        .next();
    let mut out = RenderFragment::default();
    for node in code.children() {
        if let Some(text) = node.value().as_text() {
            out.insert_text(text.to_string());
            continue;
        }
        if let Some(span) = node.value().as_element() {
            assert_eq!(span.name(), "span");
            let mut text = String::new();
            for child in node.children() {
                if let Some(text_node) = child.value().as_text() {
                    text.push_str(&text_node.to_string());
                }
            }
            refine_span(span, text, &mut out, language.clone())?;
            continue;
        }
    }
    Ok(out)
}

fn refine_span(span: &Element, text: String, snap: &mut RenderFragment, lang: Option<String>) -> Result<()> {
    let kind = convert_class_name(span).to_string();
    let node =
        RenderNode { name: lang.into_iter().chain(vec![kind].into_iter()).collect(), attributes: Default::default(), text };
    snap.insert(node);
    Ok(())
}

fn has_class(element: &Element, class: &[&str]) -> bool {
    for c in class {
        if !element.has_class(c, AsciiCaseInsensitive) {
            return false;
        }
    }
    return true;
}

fn convert_class_name(span: &Element) -> String {
    if has_class(span, &["hljs-built_in"]) {
        "builtin".to_string()
    }
    else if has_class(span, &["hljs-title", "class_"]) {
        "class".to_string()
    }
    else if has_class(span, &["hljs-title", "function_"]) {
        "method".to_string()
    }
    else {
        span.classes
            .iter()
            .filter(|f| f.starts_with("hljs"))
            .map(|s| s.strip_prefix("hljs-").unwrap_or("").to_string())
            .next()
            .unwrap_or_default()
    }
}
