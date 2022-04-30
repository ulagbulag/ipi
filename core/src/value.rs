use fixed::{traits::ToFixed, types::U0F32};
use generic_array::GenericArray;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sha2::{digest::OutputSizeUser, Sha256VarCore};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Hash(pub GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize>);

impl ::core::ops::Deref for Hash {
    type Target = GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Nonce(pub Uuid);

impl ::core::ops::Deref for Nonce {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct String<const U: usize> {
    #[serde(with = "BigArray")]
    pub buf: [u8; U],
    pub len: u64,
}

impl<const U: usize> TryFrom<::std::string::String> for String<U> {
    type Error = anyhow::Error;

    fn try_from(value: ::std::string::String) -> Result<Self, Self::Error> {
        let buf: [u8; U] = value
            .into_bytes()
            .try_into()
            .or_else(|_| bail!("Buffer Overflow"))?;
        let len = buf.len() as u64;

        Ok(Self { buf, len })
    }
}

impl<'a, const U: usize> TryFrom<&'a String<U>> for &'a str {
    type Error = ::core::str::Utf8Error;

    fn try_from(value: &'a String<U>) -> Result<Self, Self::Error> {
        ::core::str::from_utf8(&value.buf[..value.len as usize])
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct UnitInterval(pub U0F32);

impl<Src> From<Src> for UnitInterval
where
    Src: ToFixed,
{
    fn from(value: Src) -> Self {
        Self(value.checked_to_fixed().unwrap_or_default())
    }
}

impl ::core::ops::Deref for UnitInterval {
    type Target = U0F32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Value(pub u64);

impl ::core::ops::Deref for Value {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
