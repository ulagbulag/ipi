use fixed::{traits::ToFixed, types::U0F32};
use generic_array::GenericArray;
use num_traits::{FromPrimitive, ToPrimitive};
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
pub struct String<const U: usize = 256, Len = u8> {
    #[serde(with = "BigArray")]
    pub buf: [u8; U],
    pub len: Len,
}

impl<const U: usize, Len> TryFrom<::std::string::String> for String<U, Len>
where
    Len: FromPrimitive,
{
    type Error = ::anyhow::Error;

    fn try_from(value: ::std::string::String) -> Result<Self, Self::Error> {
        let buf: [u8; U] = value
            .into_bytes()
            .try_into()
            .or_else(|_| bail!("Buffer Overflow"))?;
        let len = Len::from_usize(buf.len()).ok_or_else(|| anyhow!("Buffer Overflow"))?;

        Ok(Self { buf, len })
    }
}

impl<'a, const U: usize, Len> TryFrom<&'a String<U, Len>> for &'a str
where
    Len: ToPrimitive,
{
    type Error = ::anyhow::Error;

    fn try_from(value: &'a String<U, Len>) -> Result<Self, Self::Error> {
        ::core::str::from_utf8(
            &value.buf[..value
                .len
                .to_usize()
                .ok_or_else(|| anyhow!("Buffer Overflow"))?],
        )
        .map_err(Into::into)
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
