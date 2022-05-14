use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use super::uuid::Uuid;

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct Nonce(pub Uuid);

impl From<Uuid> for Nonce {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl ::core::ops::Deref for Nonce {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::core::str::FromStr for Nonce {
    type Err = ::uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Uuid as ::core::str::FromStr>::from_str(s).map(Self)
    }
}

impl ToString for Nonce {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Nonce {
    pub fn generate() -> Self {
        Self(Uuid::generate())
    }
}
