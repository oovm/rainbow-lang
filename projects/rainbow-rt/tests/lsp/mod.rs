use arc_rs::ParserConfig;

#[test]
fn test() {
    let cfg = ParserConfig::default();
    let toc = cfg
        .parse(
            r#"
{x.1}




[c]
"#,
        )
        .unwrap()
        .toc(9);
    println!("{:#?}", toc)
}
