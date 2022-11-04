use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Archive,
    Serialize,
    Deserialize,
    ::serde::Serialize,
    ::serde::Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct U64(pub u64);

impl ::core::ops::Deref for U64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
