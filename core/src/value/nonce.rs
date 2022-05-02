use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use super::uuid::Uuid;

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct Nonce(pub Uuid);

impl ::core::ops::Deref for Nonce {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
