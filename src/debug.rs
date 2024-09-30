use crate::Serializer;
use core::fmt::{self, Debug, Formatter};
use ref_cast::RefCast;
use serde::ser::Serialize;

#[derive(RefCast)]
#[repr(transparent)]
struct Wrapper<T: ?Sized + Serialize>(T);

impl<T: ?Sized + Serialize> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.serialize(Serializer(f))?;
        Ok(())
    }
}

/// Wrap a value supporting just [`Serialize`] into [`Debug`].
pub fn debug<T: ?Sized + Serialize>(value: &T) -> &(impl ?Sized + Debug) {
    Wrapper::ref_cast(value)
}
