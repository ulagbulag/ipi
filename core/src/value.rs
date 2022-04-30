use fixed::{traits::ToFixed, types::U0F32};
use generic_array::{ArrayLength, GenericArray};
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct String<U>(pub GenericArray<char, U>)
where
    U: ArrayLength<char>;

impl<U> Copy for String<U>
where
    U: ArrayLength<char>,
    U::ArrayType: Copy,
{
}

impl<U> ::core::ops::Deref for String<U>
where
    U: ArrayLength<char>,
{
    type Target = GenericArray<char, U>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<U> Serialize for String<U>
where
    U: ArrayLength<char>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, U> Deserialize<'de> for String<U>
where
    U: ArrayLength<char>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Self as ::core::ops::Deref>::Target::deserialize(deserializer).map(Self)
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
