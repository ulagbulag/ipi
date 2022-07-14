use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Fallible, Serialize};

#[derive(Copy, Clone, Debug, Eq)]
pub struct DateTime(pub ::chrono::DateTime<::chrono::Utc>);

impl ::core::ops::Deref for DateTime {
    type Target = ::chrono::DateTime<::chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<DateTime> for <DateTime as Archive>::Archived {
    fn eq(&self, other: &DateTime) -> bool {
        self == &NaiveDateTimeTemplate::from(NaiveDateTime(other.naive_utc()))
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<DateTime> for <DateTime as Archive>::Archived {
    fn partial_cmp(&self, other: &DateTime) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&NaiveDateTimeTemplate::from(NaiveDateTime(
            other.naive_utc(),
        )))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl ::core::hash::Hash for DateTime {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
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
            .map(|datetime| datetime.to_utc())
    }
}

impl DateTime {
    pub const MIN_DATETIME: Self = DateTime(::chrono::MIN_DATETIME);
    pub const MAX_DATETIME: Self = DateTime(::chrono::MAX_DATETIME);

    pub fn now() -> Self {
        Self(::chrono::Utc::now())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NaiveDateTime(pub ::chrono::NaiveDateTime);

impl From<::chrono::NaiveDateTime> for NaiveDateTime {
    fn from(value: ::chrono::NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<NaiveDateTime> for NaiveDateTimeTemplate {
    fn from(value: NaiveDateTime) -> Self {
        Self {
            secs: value.timestamp(),
            nanos: value.timestamp_subsec_nanos(),
        }
    }
}

impl TryFrom<NaiveDateTimeTemplate> for NaiveDateTime {
    type Error = ::core::convert::Infallible;

    fn try_from(value: NaiveDateTimeTemplate) -> Result<Self, Self::Error> {
        // FIXME: handle chrono input errors
        Ok(Self(
            ::chrono::NaiveDateTime::from_timestamp_opt(value.secs, value.nanos).unwrap(),
        ))
    }
}

impl ::core::ops::Deref for NaiveDateTime {
    type Target = ::chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::core::str::FromStr for NaiveDateTime {
    type Err = ::chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <::chrono::NaiveDateTime as ::core::str::FromStr>::from_str(s).map(Self)
    }
}

impl ToString for NaiveDateTime {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Archive for NaiveDateTime {
    type Archived = <NaiveDateTimeTemplate as Archive>::Archived;
    type Resolver = <NaiveDateTimeTemplate as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        NaiveDateTimeTemplate::from(*self).resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for NaiveDateTime {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        NaiveDateTimeTemplate::from(*self).serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<NaiveDateTime, D> for <NaiveDateTime as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<NaiveDateTime, D::Error> {
        Deserialize::<NaiveDateTimeTemplate, D>::deserialize(self, deserializer)
            // FIXME: handle chrono input errors
            .map(|e| e.try_into().unwrap())
    }
}

impl NaiveDateTime {
    pub fn to_utc(&self) -> DateTime {
        DateTime(::chrono::DateTime::from_utc(self.0, ::chrono::Utc))
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
// FIXME: hide it
pub struct NaiveDateTimeTemplate {
    secs: i64,
    nanos: u32,
}
