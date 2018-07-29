use serde::ser::Serialize;
use std::fmt::{self, Debug, Formatter};
use Serializer;

pub struct Wrapper<'a, T: 'a + ?Sized + Serialize>(pub &'a T);

impl<'a, T: 'a + ?Sized + Serialize> Debug for Wrapper<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Serialize::serialize(self.0, Serializer(f))?;
        Ok(())
    }
}
