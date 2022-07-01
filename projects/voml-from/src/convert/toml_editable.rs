use crate::ast::ASTKind;
use toml_edit::{Array, ArrayOfTables, Document, Item, Table, Value};

impl From<Document> for ASTKind {
    fn from(v: Document) -> Self {
        for (i, j) in v.iter() {
            println!("{:?}", i);
            println!("{:?}", j);
        }
        unimplemented!()
    }
}

// impl From<Item> for ASTKind {
//     fn from(v: Item) -> Self {
//         match v {
//             Item::None => String::from("null"),
//             Item::Value(value) => value.to_arc(),
//             Item::Table(table) => table.to_arc(),
//             Item::ArrayOfTables(array) => array.to_arc(),
//         }
//     }
// }
//
// impl From<Value> for ASTKind {
//     fn from(v: Value) -> Self {
//         match v {
//             Value::Integer(i) => format!("{}", i.value()),
//             Value::String(s) => format!("{:#?}", s.value()),
//             Value::Float(f) => format!("{}", f.value()),
//             Value::DateTime(d) => format!("\"{}\"", d.value()),
//             Value::Boolean(b) => format!("{}", b.value()),
//             Value::Array(a) => format!("{}", a.to_arc()),
//             Value::InlineTable(_) => {
//                 println!("{:#?}", self);
//                 unimplemented!()
//             }
//         }
//     }
// }
//
// impl From<Table> for ASTKind {
//
// }
//
// impl From<Array>
//
// impl ToArc for Table {
//     fn to_arc(&self) -> String {
//         let mut pairs = vec![];
//         for (k, v) in self.iter() {
//             pairs.push(format!("{} = {}", k, v.to_arc()))
//         }
//         build_dict(pairs)
//     }
// }
//
// impl ToArc for Array {
//     fn to_arc(&self) -> String {
//         let mut terms = vec![];
//         for i in self.iter() {
//             terms.push(i.to_arc())
//         }
//         build_list(terms)
//     }
// }
//
// impl ToArc for ArrayOfTables {
//     fn to_arc(&self) -> String {
//         let mut terms = vec![];
//         for i in self.iter() {
//             terms.push(i.to_arc())
//         }
//         build_list(terms)
//     }
// }
