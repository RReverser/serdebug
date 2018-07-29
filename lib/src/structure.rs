use debug::Wrapper;
use error::Error;
use serde::ser::{Serialize, SerializeStruct, SerializeStructVariant};
use std::fmt::{DebugStruct, Formatter};

pub struct Serializer<'a, 'b: 'a>(DebugStruct<'a, 'b>);

impl<'a, 'b: 'a> Serializer<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>, name: &str) -> Self {
        Serializer(f.debug_struct(name))
    }
}

impl<'a, 'b: 'a> SerializeStruct for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.0.field(key, &Wrapper(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}

impl<'a, 'b: 'a> SerializeStructVariant for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<(), Error> {
        SerializeStruct::end(self)
    }
}
