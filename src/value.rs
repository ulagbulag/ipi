use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Value(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Nonce(pub Uuid);
