use bytecheck::CheckBytes;
use num_traits::ToPrimitive;
use rkyv::{Archive, Deserialize, Serialize};

use super::string::String;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(archive = "
    <String<U, Len> as Archive>::Archived: Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
",))]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct Text<const U: usize = 256, Len = u8> {
    pub msg: String<U, Len>,
    pub lang: LanguageTag,
}

impl<const U: usize, Len> ::core::fmt::Debug for Text<U, Len>
where
    Len: ToPrimitive,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.msg, f)
    }
}

impl<const U: usize, Len> ::core::fmt::Display for Text<U, Len>
where
    Len: ToPrimitive,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.msg, f)
    }
}

impl<const U: usize, Len> Text<U, Len> {
    pub fn with_en_us(msg: String<U, Len>) -> Self {
        Self {
            msg,
            lang: LanguageTag::new_en_us(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct LanguageTag(String<5, u8>);

impl TryFrom<&str> for LanguageTag {
    type Error = ::anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use ::core::str::FromStr;

        ::language_tags::LanguageTag::from_str(value)
            .map_err(Into::into)
            .map(|e| e.to_string())
            .and_then(TryInto::try_into)
            .map(Self)
    }
}

impl ::core::ops::Deref for LanguageTag {
    type Target = String<5, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LanguageTag {
    pub fn new_en_us() -> Self {
        "en-us".try_into().unwrap()
    }
}
