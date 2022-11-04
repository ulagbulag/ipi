use bytecheck::CheckBytes;
use rkyv::{ser::Serializer, string::ArchivedString, Archive, Deserialize, Fallible, Serialize};

use super::hash::Hash;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    Archive,
    Serialize,
    Deserialize,
    ::serde::Serialize,
    ::serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct Text {
    pub msg: String,
    pub lang: LanguageTag,
}

impl ::core::fmt::Debug for Text {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.msg, f)
    }
}

impl ::core::fmt::Display for Text {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.msg, f)
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::with_en_us(String::default())
    }
}

impl Text {
    pub fn with_en_us(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
            lang: LanguageTag::new_en_us(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct TextHash {
    pub msg: Hash,
    pub lang: Hash,
}

impl From<Text> for TextHash {
    fn from(value: Text) -> Self {
        Self {
            msg: Hash::with_str(&value.msg),
            lang: Hash::with_str(&value.lang.to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)]
pub struct LanguageTag(::language_tags::LanguageTag);

impl ::core::str::FromStr for LanguageTag {
    type Err = ::language_tags::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::language_tags::LanguageTag::from_str(s)
            .map_err(Into::into)
            .map(Self)
    }
}

impl ::core::ops::Deref for LanguageTag {
    type Target = ::language_tags::LanguageTag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<LanguageTag> for ArchivedString {
    fn eq(&self, other: &LanguageTag) -> bool {
        self.as_ref() == other.0.as_str()
    }
}

impl PartialOrd<LanguageTag> for ArchivedString {
    fn partial_cmp(&self, other: &LanguageTag) -> Option<::core::cmp::Ordering> {
        self.as_ref().partial_cmp(other.0.as_str())
    }
}

impl Archive for LanguageTag {
    type Archived = <String as Archive>::Archived;
    type Resolver = <String as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.to_string().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for LanguageTag
where
    S: Serializer,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<LanguageTag, D> for <LanguageTag as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<LanguageTag, D::Error> {
        Deserialize::<String, D>::deserialize(self, deserializer)
            // FIXME: handle chrono input errors
            .map(|ref e| e.parse().unwrap())
    }
}

impl LanguageTag {
    pub fn new_en_us() -> Self {
        "en-us".parse().unwrap()
    }
}
