use serde::ser;
use std::fmt;

/// A [`ser::Error`]-compatible wrapper for [`fmt::Error`].
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] fmt::Error);

impl From<Error> for fmt::Error {
    fn from(err: Error) -> fmt::Error {
        err.0
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        unimplemented!("This type is intended to be used only for fmt::Error")
    }
}
