pub mod chrono;
pub mod hash;
pub mod nonce;
pub mod primitives;
pub mod text;
pub mod uuid;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub enum ValueType {
    None,
    Dyn,
    Bool,
    I64,
    U64,
    F32,
    String,
    Text,
}
