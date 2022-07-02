use rainbow_core::Schema;
use std::str::FromStr;

#[test]
fn parse() {
    let schema = Schema::from_str(include_str!("neo.rmb")).unwrap();
    println!("{:#?}", schema);
}
