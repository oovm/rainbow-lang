#[macro_use]
extern crate serde_arc as arc;

use arc::{parse, Arc, Getter};

#[test]
fn test_list() {
    let l = list![1, 2, 3, 1.0, 2.0, 3f64, list![false, true]];
    println!("{}", l[1]);
    println!("{:?}", l.get(1));
}

#[test]
fn test_dict() {
    let d = parse(include_str!("../parser/json_compatibility/package.json"));
    println!("{}", d);
}
