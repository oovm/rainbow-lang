use arc_rs::utils::parse_json;

#[test]
fn json() {
    let json = include_str!("test.json");
    let v = parse_json(json).unwrap();
    println!("{:#?}", v)
}
