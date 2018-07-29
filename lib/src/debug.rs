use serde::ser::Serialize;
use std::fmt::{self, Debug, Formatter};
use Serializer;

pub(crate) struct Wrapper<'a, T: 'a + ?Sized + Serialize>(pub &'a T);

impl<'a, T: 'a + ?Sized + Serialize> Debug for Wrapper<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.serialize(Serializer(f))?;
        Ok(())
    }
}
