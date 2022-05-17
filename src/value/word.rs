use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use super::{
    hash::Hash,
    text::{Text, TextHash},
};

#[derive(Clone, Default, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct Word {
    pub kind: String,
    pub text: Text,
}

impl ::core::fmt::Debug for Word {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.text, f)
    }
}

impl ::core::fmt::Display for Word {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.text, f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct WordHash {
    pub kind: Hash,
    pub text: TextHash,
}

impl From<Word> for WordHash {
    fn from(value: Word) -> Self {
        Self {
            kind: Hash::with_str(&value.kind),
            text: value.text.into(),
        }
    }
}
