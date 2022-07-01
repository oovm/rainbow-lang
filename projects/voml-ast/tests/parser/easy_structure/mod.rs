use super::*;

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let ast = parse_text(include_str!(concat!($file_name, ".arc"))).unwrap();
        assert_eq!(include_str!(concat!($file_name, ".out.arc")), format!("{:#?}", Value::from(ast)))
    }
    };
}

run_test![empty, basic, scope, number, cite, list_scope,];
