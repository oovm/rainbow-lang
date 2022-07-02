use arc_pest::ParserConfig;

#[test]
fn parse() {
    let parser = ParserConfig::default();
    println!("{:#?}",parser.parse(include_str!("neo.rmb")).unwrap() );
}