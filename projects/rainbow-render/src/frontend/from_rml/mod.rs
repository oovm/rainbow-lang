use html_parser::Dom;

#[test]
fn main() {
    let html = include_str!("main.rml");
    let dom = Dom::parse(html).unwrap();
    println!("{:#?}", Dom::parse(html).unwrap());
}
