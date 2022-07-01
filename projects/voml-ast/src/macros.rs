/// list![1, 2, 3]
#[macro_export]
macro_rules! list {
    [] => ($crate::value::List::empty());
    [$($item:expr), *] => ({
        let size = 0 $( + {let _ = $item; 1} )*;
        let mut list = Vec::with_capacity(size);
        $(list.push($crate::Value::from($item));)*
        $crate::Value::from(list)
    })
}

/// dict! {
///     a: null
/// }
#[macro_export]
macro_rules! dict {
    {} => ($crate::value::Dict::empty());
    {$($key:tt: $value:expr), *} => {
        $crate::dict!($($key: $value,)*)
    };
    {$($key:tt: $value:expr,)*} => ({
        let size = 0 $( + {let _ = $key; 1} )*;
        let mut dict = $crate::utils::IndexMap::with_capacity(size);
        $(dict.insert(String::from($key), $crate::Value::from($value));)*
        $crate::Value::from(dict)
    })
}

#[test]
fn test() {
    let a = list![];
    println!("{:?}", a);
    let a = list![1];
    println!("{:?}", a);
    let a = list![1, 2];
    println!("{:?}", a);
    let a = dict! {};
    println!("{:?}", a);
    let a = dict! {
        "x": 1
    };
    println!("{:?}", a);
    let a = dict! {
        "x": list![1,2,3]
    };
    println!("{:?}", a);
}
