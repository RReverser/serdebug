use serde::ser::Serialize;
use std::fmt::{self, Debug, Formatter};
use Serializer;

struct Wrapper<T: Serialize>(T);

impl<T: Serialize> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.serialize(Serializer(f))?;
        Ok(())
    }
}

/// Wrap a value supporting just [`Serialize`] into [`Debug`].
pub fn debug<'a, T: ?Sized + Serialize>(value: &'a T) -> impl Debug + 'a {
    Wrapper(value)
}
