use super::*;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use crate::ASTKind;

pub struct ArrayBuffer<'s> {
    ptr: &'s mut ReadableConfigSerializer,
    name: Option<&'static str>,
    buffer: Vec<AST>,
}

impl<'s> ArrayBuffer<'s> {
    pub fn new(ptr: &'s mut ReadableConfigSerializer, name: Option<&'static str>, size: usize) -> Self {
        Self { name, ptr, buffer: Vec::with_capacity(size) }
    }
    fn push_sequence<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ptr)?;
        self.buffer.push(self.ptr.this.to_owned());
        Ok(())
    }
}

impl<'a> SerializeSeq for ArrayBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = ASTKind::list(self.buffer).into())
    }
}

// Same thing but for tuples.
impl<'a> SerializeTuple for ArrayBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = ASTKind::list(self.buffer).into())
    }
}

// Same thing but for tuple structs.
impl<'a> SerializeTupleStruct for ArrayBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = ASTKind::list(self.buffer).into())
    }
}

impl<'a> SerializeTupleVariant for ArrayBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = ASTKind::list(self.buffer).into())
    }
}
