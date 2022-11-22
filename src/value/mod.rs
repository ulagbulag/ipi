pub mod array;
pub mod chrono;
pub mod hash;
pub mod nonce;
pub mod primitives;
pub mod text;
pub mod unit_interval;
pub mod uuid;

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
pub enum Value {
    None,
    Dyn,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bytes(Vec<u8>),
    String(String),
    Text(self::text::Text),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::None => "()".to_string(),
            Self::Dyn => "{}".to_string(),
            Self::Bool(value) => value.to_string(),
            Self::I8(value) => value.to_string(),
            Self::I16(value) => value.to_string(),
            Self::I32(value) => value.to_string(),
            Self::I64(value) => value.to_string(),
            Self::U8(value) => value.to_string(),
            Self::U16(value) => value.to_string(),
            Self::U32(value) => value.to_string(),
            Self::U64(value) => value.to_string(),
            Self::F32(value) => value.to_string(),
            Self::F64(value) => value.to_string(),
            Self::Bytes(value) => value.to_base58(),
            Self::String(value) => value.to_string(),
            Self::Text(value) => value.to_string(),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Archive,
    Serialize,
    Deserialize,
    ::serde::Serialize,
    ::serde::Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub enum ValueType {
    None,
    Dyn,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bytes,
    String,
    Text,
}
