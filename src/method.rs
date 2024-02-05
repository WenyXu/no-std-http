use strum::{AsRefStr, Display, EnumIter, EnumString};
/// The Request Method (VERB)
///
/// This type also contains constants for a number of common HTTP methods such
/// as GET, POST, etc.
///
/// Currently includes 8 variants representing the 8 methods defined in
/// [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH.
///
/// # Examples
///
/// ```
/// use http::Method;
/// use std::str::FromStr;
///
/// assert_eq!(Method::Get, Method::from_str("GET").unwrap());
/// assert!(Method::Get.is_idempotent());
/// assert_eq!(Method::Post.as_ref(), "POST");
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, AsRefStr, Display, EnumString, EnumIter)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum Method {
    Options,
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl Method {
    /// Whether a method is considered "safe", meaning the request is
    /// essentially read-only.
    ///
    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.1)
    /// for more words.
    pub fn is_safe(&self) -> bool {
        use Method::*;
        matches!(self, Get | Head | Options | Trace)
    }

    /// Whether a method is considered "idempotent", meaning the request has
    /// the same result if executed multiple times.
    ///
    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.2) for
    /// more words.
    pub fn is_idempotent(&self) -> bool {
        use Method::*;
        match self {
            Put | Delete => true,
            _ => self.is_safe(),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_enum_string() {
        for method in Method::iter() {
            let str = method.as_ref();
            assert_eq!(Method::from_str(str).unwrap(), method);
        }
    }

    #[test]
    fn test_is_idempotent() {
        assert!(Method::Options.is_idempotent());
        assert!(Method::Get.is_idempotent());
        assert!(Method::Put.is_idempotent());
        assert!(Method::Delete.is_idempotent());
        assert!(Method::Head.is_idempotent());
        assert!(Method::Trace.is_idempotent());

        assert!(!Method::Post.is_idempotent());
        assert!(!Method::Connect.is_idempotent());
        assert!(!Method::Patch.is_idempotent());
    }
}
