use core::ops;

use base58::ToBase58;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Archive,
    Serialize,
    Deserialize,
    ::serde::Serialize,
    ::serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct Bytes(pub Vec<u8>);

impl ops::Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToBase58 for Bytes {
    fn to_base58(&self) -> String {
        self.0.to_base58()
    }
}

impl ToString for Bytes {
    fn to_string(&self) -> String {
        self.to_base58()
    }
}
