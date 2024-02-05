//! HTTP version
//!
//! This module contains a definition of the `Version` type. The `Version`
//! type is intended to be accessed through the root of the crate
//! (`http::Version`) rather than this module.
//!
//! The `Version` type contains constants that represent the various versions
//! of the HTTP protocol.
//!
//! # Examples
//!
//! ```
//! use http::Version;
//!
//! let http11 = Version::Http11;
//! let http2 = Version::H2;
//! assert!(http11 != http2);
//!
//! println!("{:?}", http2);
//! ```

use strum::AsRefStr;

/// Represents a version of the HTTP spec.
#[derive(Debug, Default, PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash, AsRefStr)]
pub enum Version {
    #[strum(serialize = "HTTP/0.9")]
    Http09,
    #[strum(serialize = "HTTP/1.0")]
    Http10,
    #[default]
    #[strum(serialize = "HTTP/1.1")]
    Http11,
    #[strum(serialize = "HTTP/2.0")]
    H2,
    #[strum(serialize = "HTTP/3.0")]
    H3,
}
