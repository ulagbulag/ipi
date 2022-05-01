use fixed::{traits::ToFixed, types::U0F32};
use generic_array::GenericArray;
use num_traits::{FromPrimitive, ToPrimitive};
use rkyv::{Archive, Deserialize, Fallible, Serialize};
use sha2::{digest::OutputSizeUser, Sha256VarCore};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DateTime(pub ::chrono::DateTime<::chrono::Utc>);

impl ::core::ops::Deref for DateTime {
    type Target = ::chrono::DateTime<::chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Archive for DateTime {
    type Archived = <NaiveDateTime as Archive>::Archived;
    type Resolver = <NaiveDateTime as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        NaiveDateTime(self.naive_utc()).resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for DateTime {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        NaiveDateTime(self.naive_utc()).serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<DateTime, D> for <DateTime as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<DateTime, D::Error> {
        Deserialize::<NaiveDateTime, D>::deserialize(self, deserializer)
            .map(|datetime| ::chrono::DateTime::from_utc(datetime.0, ::chrono::Utc))
            .map(DateTime)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NaiveDateTime(pub ::chrono::NaiveDateTime);

impl ::core::ops::Deref for NaiveDateTime {
    type Target = ::chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
// FIXME: hide it
pub struct NaiveDateTimeTemplate {
    secs: i64,
    nanos: u32,
}

impl Archive for NaiveDateTime {
    type Archived = <NaiveDateTimeTemplate as Archive>::Archived;
    type Resolver = <NaiveDateTimeTemplate as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let template = NaiveDateTimeTemplate {
            secs: self.timestamp(),
            nanos: self.timestamp_subsec_nanos(),
        };
        template.resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for NaiveDateTime {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let template = NaiveDateTimeTemplate {
            secs: self.timestamp(),
            nanos: self.timestamp_subsec_nanos(),
        };
        template.serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<NaiveDateTime, D> for <NaiveDateTime as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<NaiveDateTime, D::Error> {
        Deserialize::<NaiveDateTimeTemplate, D>::deserialize(self, deserializer)
            .map(|template| {
                // FIXME: handle chrono input errors
                ::chrono::NaiveDateTime::from_timestamp_opt(template.secs, template.nanos).unwrap()
            })
            .map(NaiveDateTime)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Hash(pub GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize>);

impl ::core::ops::Deref for Hash {
    type Target = GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Archive for Hash {
    type Archived = <[u8; 32] as Archive>::Archived;
    type Resolver = <[u8; 32] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        <[u8; 32]>::from(self.0).resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Hash {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        <[u8; 32]>::from(self.0).serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Hash, D> for <Hash as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Hash, D::Error> {
        Deserialize::<[u8; 32], D>::deserialize(self, deserializer)
            .map(Into::into)
            .map(Hash)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Nonce(pub Uuid);

impl ::core::ops::Deref for Nonce {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Archive for Nonce {
    type Archived = <u128 as Archive>::Archived;
    type Resolver = <u128 as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.as_u128().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Nonce {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.as_u128().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Nonce, D> for <Nonce as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Nonce, D::Error> {
        Deserialize::<u128, D>::deserialize(self, deserializer)
            .map(Uuid::from_u128)
            .map(Nonce)
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct String<const U: usize = 256, Len = u8>
where
    <[u8; U] as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    Len: Archive,
    <Len as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    pub buf: [u8; U],
    pub len: Len,
}

impl<const U: usize, Len> TryFrom<::std::string::String> for String<U, Len>
where
    <[u8; U] as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    Len: Archive + FromPrimitive,
    <Len as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
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
    <[u8; U] as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    Len: Archive + ToPrimitive,
    <Len as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Archive for UnitInterval {
    type Archived = <u32 as Archive>::Archived;
    type Resolver = <u32 as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.to_bits().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for UnitInterval {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.to_bits().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<UnitInterval, D> for <UnitInterval as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<UnitInterval, D::Error> {
        Deserialize::<u32, D>::deserialize(self, deserializer)
            .map(U0F32::from_bits)
            .map(UnitInterval)
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
#[repr(transparent)]
pub struct Value(pub u64);

impl ::core::ops::Deref for Value {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
