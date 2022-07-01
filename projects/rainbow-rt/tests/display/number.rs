#[macro_use]
extern crate arc_ast;

use arc_ast::Arc;

#[test]
fn test() {
    let s = Arc::new_number("i32", "2147483647");
    println!("{}", s);
}

#[test]
fn test_index() {
    let l = list![1, 2, 3, 1.0, 2.0, 3f64, list![false, true]];
    println!("{}", l[1]);
    println!("{}", l[-1]);
}
