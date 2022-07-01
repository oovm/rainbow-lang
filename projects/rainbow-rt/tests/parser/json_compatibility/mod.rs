use super::*;

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let ast = parse_text(include_str!(concat!($file_name, ".json"))).unwrap();
        println!("{:#?}", Value::from(ast))
    }
    };
}

run_test![number, string, object, package,];
