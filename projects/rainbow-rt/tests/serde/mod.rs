use serde::Serialize;
use arc_rs::ReadableConfigSerializer;
use arc_rs::Value;
use std::collections::HashMap;


#[test]
fn test_primitive() {
    let mut s = ReadableConfigSerializer::default();

    let v= Value::from(s.serialize(0usize).unwrap());
    assert_eq!(format!("{:?}", v), "0");

    let v= Value::from(s.serialize("0").unwrap());
    assert_eq!(format!("{:?}", v), "\"0\"");

    let v= Value::from(s.serialize('0').unwrap());
    assert_eq!(format!("{:?}", v), "\"0\"");

    let v= Value::from(s.serialize(true).unwrap());
    assert_eq!(format!("{:?}", v), "true");
}


#[test]
fn test_list() {
    let mut s = ReadableConfigSerializer::default();

    let test = ();
    let v= Value::from(s.serialize(test).unwrap());
    assert_eq!(format!("{:?}", v), "0");

    let test = vec![0];
    let v= Value::from(s.serialize(test).unwrap());
    assert_eq!(format!("{:?}", v), "0");

    let test = vec![vec![0], vec![1]];
    let v= Value::from(s.serialize(test).unwrap());
    assert_eq!(format!("{:?}", v), "0");

    let test = (vec![0], vec![1]);
    let v= Value::from(s.serialize(test).unwrap());
    assert_eq!(format!("{:?}", v), "0");
}

#[test]
fn test_dict() {
    let mut s = ReadableConfigSerializer::default();

    let mut map = HashMap::new();
    map.insert(0, true);
    let v= Value::from(s.serialize(map).unwrap());
    assert_eq!(format!("{:?}", v), "0");
}


#[derive(Serialize)]
struct TestTuple(usize, Vec<usize>);

#[derive(Serialize)]
struct TestStruct {
    int: usize,
    seq: Vec<usize>,
}

#[test]
fn test_named() {
    let mut s = ReadableConfigSerializer::default();

    let test = TestStruct { int: 0, seq: vec![1, 2] };
    let v= Value::from(s.serialize(test).unwrap());
    assert_eq!(format!("{:?}", v), "0");

    let test = TestTuple(0, vec![1, 2]);
    let v= Value::from(s.serialize(test).unwrap());
    assert_eq!(format!("{:?}", v), "0");
}

// //
// // #[test]
// // fn test_enum() {
// //     #[derive(Serialize)]
// //     enum E {
// //         Unit,
// //         Newtype(u32),
// //         Tuple(u32, u32),
// //         Struct { a: u32 },
// //     }
// //
// //     let u = E::Unit;
// //     let expected = r#""Unit""#;
// //     assert_eq!(serialize(&u).unwrap(), expected);
// //
// //     let n = E::Newtype(1);
// //     let expected = r#"{"Newtype":1}"#;
// //     assert_eq!(serialize(&n).unwrap(), expected);
// //
// //     let t = E::Tuple(1, 2);
// //     let expected = r#"{"Tuple":[1,2]}"#;
// //     assert_eq!(serialize(&t).unwrap(), expected);
// //
// //     let s = E::Struct { a: 1 };
// //     let expected = r#"{"Struct":{"a":1}}"#;
// //     assert_eq!(serialize(&s).unwrap(), expected);
// // }
