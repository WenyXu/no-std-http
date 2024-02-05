#![no_std]

// Modified from https://github.com/hyperium/http/blob/f446500e157b827fa8d2819c005734547abc2b39
// ! A library of common HTTP types design for no std environment.

extern crate alloc;

mod byte_str;
mod convert;
pub mod error;
pub mod header;
pub mod method;
pub mod request;
pub mod response;
pub mod status;
pub mod uri;
pub mod version;

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status::StatusCode;
pub use uri::Uri;
pub use version::Version;
