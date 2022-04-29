use chrono::{DateTime, Utc};

use crate::{account::Identity, value::Nonce};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Metadata<T> {
    pub nonce: Nonce,
    pub created_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub target: Option<Identity>,
    pub data: T,
}
