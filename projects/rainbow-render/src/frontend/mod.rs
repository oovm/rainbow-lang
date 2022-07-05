mod from_hljs;
mod from_rml;

#[cfg(feature = "html")]
fn parse_html(html: &str) -> rainbow_core::Result<html_parser::Dom> {
    let dom = match html_parser::Dom::parse(html) {
        Ok(o) => o,
        Err(e) => return Err(rainbow_core::RainbowError::syntax_error(e.to_string())),
    };
    if !dom.errors.is_empty() {
        return Err(rainbow_core::RainbowError::syntax_error(dom.errors[0].to_string()));
    }
    return Ok(dom);
}
