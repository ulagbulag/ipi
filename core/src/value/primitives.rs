use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct U64(pub u64);

impl ::core::ops::Deref for U64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
