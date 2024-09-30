use crate::{debug, Error};
use core::fmt::{DebugMap, Formatter};
use serde::ser::{Serialize, SerializeMap};

pub struct Serializer<'a, 'b: 'a>(DebugMap<'a, 'b>);

impl<'a, 'b: 'a> Serializer<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>) -> Self {
        Serializer(f.debug_map())
    }
}

impl<'a, 'b: 'a> SerializeMap for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        self.0.key(&debug(key));
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.value(&debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}
