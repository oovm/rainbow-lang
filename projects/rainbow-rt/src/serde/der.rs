use crate::Value;
use indexmap::map::IndexMap;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::fmt;

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Error")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(value.into())
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: ::serde::de::SeqAccess<'de>,
            {
                let mut list: Vec<Value> = vec![];

                while let Some(elem) = visitor.next_element()? {
                    list.push(elem);
                }

                Ok(list.into())
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: ::serde::de::MapAccess<'de>,
            {
                let mut dict: IndexMap<String, Value> = IndexMap::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    dict.insert(key, value);
                }

                Ok(dict.into())
            }
        }
        deserializer.deserialize_any(ValueVisitor)
    }
}
