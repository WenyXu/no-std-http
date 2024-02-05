use core::fmt;

use crate::uri;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Uri(uri::InvalidUri),

    UriParts(uri::InvalidUriParts),
    InvalidStatusCode,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidStatusCode => f.write_str("invalid status code"),
            Error::UriParts(e) => write!(f, "{e}"),
            Error::Uri(e) => write!(f, "{e}"),
        }
    }
}

impl From<uri::InvalidUri> for Error {
    fn from(err: uri::InvalidUri) -> Error {
        Error::Uri(err)
    }
}

impl From<uri::InvalidUriParts> for Error {
    fn from(err: uri::InvalidUriParts) -> Error {
        Error::UriParts(err)
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(err: core::convert::Infallible) -> Error {
        match err {}
    }
}
