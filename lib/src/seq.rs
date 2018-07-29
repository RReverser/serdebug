use error::Error;
use serde::ser::{Serialize, SerializeSeq};
use std::fmt::{DebugList, Formatter};
use debug::Wrapper;

pub struct Serializer<'a, 'b: 'a>(DebugList<'a, 'b>);

impl<'a, 'b: 'a> Serializer<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>) -> Self {
        Serializer(f.debug_list())
    }
}

impl<'a, 'b: 'a> SerializeSeq for Serializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.entry(&Wrapper(value));
        Ok(())
    }

    fn end(mut self) -> Result<(), Error> {
        Ok(self.0.finish()?)
    }
}
