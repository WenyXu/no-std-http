use core::fmt::Debug;

use crate::{header::HeaderMap, version::Version, Method, Uri};

/// Component parts of an HTTP `Request`
///
/// The HTTP request head consists of a method, uri, version, and a set of
/// header fields.
#[derive(Debug, Default, Clone)]
pub struct Request<T> {
    /// The request's method
    pub method: Method,

    /// The request's URI
    pub uri: Uri,

    /// The request's version
    pub version: Version,

    /// The request's headers
    pub headers: HeaderMap,

    /// The request's body
    pub body: T,
}
