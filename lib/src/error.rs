use serde::ser;
use std::error;
use std::fmt::{self, Formatter};

/// A [`ser::Error`]-compatible wrapper for [`fmt::Error`].
pub struct Error(fmt::Error);

impl From<Error> for fmt::Error {
    fn from(err: Error) -> fmt::Error {
        err.0
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error(err)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        unimplemented!("This type is intended to be used only for fmt::Error")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(&self.0)
    }
}
