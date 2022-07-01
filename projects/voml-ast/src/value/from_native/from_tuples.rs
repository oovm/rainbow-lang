use crate::{value::List, Value};

impl From<()> for Value {
    fn from(_: ()) -> Self {
        List::empty()
    }
}

// impl<T1> From<(T1,)> for Value
// where
//     Value: From<T1>,
// {
//     fn from(v: (T1,)) -> Self {
//         let mut list = Vec::with_capacity(1);
//         list.push(Value::from(v.0));
//         Value::from(list)
//     }
// }

impl<T1, T2> From<(T1, T2)> for Value
where
    Value: From<T1>,
    Value: From<T2>,
{
    fn from(v: (T1, T2)) -> Self {
        let mut list = Vec::with_capacity(2);
        list.push(Value::from(v.0));
        list.push(Value::from(v.1));
        Value::from(list)
    }
}

impl<T1, T2, T3> From<(T1, T2, T3)> for Value
where
    Value: From<T1>,
    Value: From<T2>,
    Value: From<T3>,
{
    fn from(v: (T1, T2, T3)) -> Self {
        let mut list = Vec::with_capacity(3);
        list.push(Value::from(v.0));
        list.push(Value::from(v.1));
        list.push(Value::from(v.2));
        Value::from(list)
    }
}

impl<T1, T2, T3, T4> From<(T1, T2, T3, T4)> for Value
where
    Value: From<T1>,
    Value: From<T2>,
    Value: From<T3>,
    Value: From<T4>,
{
    fn from(v: (T1, T2, T3, T4)) -> Self {
        let mut list = Vec::with_capacity(4);
        list.push(Value::from(v.0));
        list.push(Value::from(v.1));
        list.push(Value::from(v.2));
        list.push(Value::from(v.3));
        Value::from(list)
    }
}

impl<T1, T2, T3, T4, T5> From<(T1, T2, T3, T4, T5)> for Value
where
    Value: From<T1>,
    Value: From<T2>,
    Value: From<T3>,
    Value: From<T4>,
    Value: From<T5>,
{
    fn from(v: (T1, T2, T3, T4, T5)) -> Self {
        let mut list = Vec::with_capacity(5);
        list.push(Value::from(v.0));
        list.push(Value::from(v.1));
        list.push(Value::from(v.2));
        list.push(Value::from(v.3));
        list.push(Value::from(v.4));
        Value::from(list)
    }
}

impl<T1, T2, T3, T4, T5, T6> From<(T1, T2, T3, T4, T5, T6)> for Value
where
    Value: From<T1>,
    Value: From<T2>,
    Value: From<T3>,
    Value: From<T4>,
    Value: From<T5>,
    Value: From<T6>,
{
    fn from(v: (T1, T2, T3, T4, T5, T6)) -> Self {
        let mut list = Vec::with_capacity(6);
        list.push(Value::from(v.0));
        list.push(Value::from(v.1));
        list.push(Value::from(v.2));
        list.push(Value::from(v.3));
        list.push(Value::from(v.4));
        list.push(Value::from(v.5));
        Value::from(list)
    }
}
