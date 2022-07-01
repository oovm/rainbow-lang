use super::*;

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let ast = parse_yaml(include_str!(concat!($file_name, ".yaml"))).unwrap();
        assert_eq!(include_str!(concat!($file_name, ".out.arc")), format!("{:#?}", Value::from(ast)))
    }
    };
}

run_test![
    easy_1, easy_2, easy_3, easy_4, easy_5, // normal_1,
    normal_2, normal_3, normal_4, normal_5, hard_1,
];
