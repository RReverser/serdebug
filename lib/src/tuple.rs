use debug::debug;
use error::Error;
use serde::ser::{Serialize, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use std::fmt::{DebugTuple, Formatter};

pub struct Serializer<'a, 'b: 'a>(DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> Serializer<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>, name: &str) -> Self {
        Serializer(f.debug_tuple(name))
    }
}

impl<'a, 'b: 'a> SerializeTuple for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.field(&debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}

impl<'a, 'b: 'a> SerializeTupleStruct for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        SerializeTuple::end(self)
    }
}

impl<'a, 'b: 'a> SerializeTupleVariant for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        SerializeTuple::end(self)
    }
}
