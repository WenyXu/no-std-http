use alloc::collections::BTreeMap;
use alloc::string::String;
use smallvec::SmallVec;

pub type HeaderMap = BTreeMap<String, SmallVec<[String; 1]>>;
