use debug::Wrapper;
use error::Error;
use serde::ser::{Serialize, SerializeMap};
use std::fmt::{DebugMap, Formatter};

pub struct Serializer<'a, 'b: 'a>(DebugMap<'a, 'b>);

impl<'a, 'b: 'a> Serializer<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>) -> Self {
        Serializer(f.debug_map())
    }
}

impl<'a, 'b: 'a> SerializeMap for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        unimplemented!("Only entries are supported in DebugMap")
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        unimplemented!("Only entries are supported in DebugMap")
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        self.0.entry(&Wrapper(key), &Wrapper(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}
