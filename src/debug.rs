use crate::Serializer;
use serde::ser::Serialize;
use std::fmt::{self, Debug, Formatter};

struct Wrapper<T: Serialize>(T);

impl<T: Serialize> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.serialize(Serializer(f))?;
        Ok(())
    }
}

/// Wrap a value supporting just [`Serialize`] into [`Debug`].
pub fn debug<T: ?Sized + Serialize>(value: &T) -> impl Debug + '_ {
    Wrapper(value)
}
