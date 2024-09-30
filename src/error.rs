use core::fmt;
use serde::ser;

/// A [`ser::Error`]-compatible wrapper for [`fmt::Error`].
#[derive(Debug)]
pub struct Error(fmt::Error);

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error(err)
    }
}

impl From<Error> for fmt::Error {
    fn from(err: Error) -> fmt::Error {
        err.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        unimplemented!("This type is intended to be used only for fmt::Error")
    }
}
