use chrono::{DateTime, Utc};

use crate::{
    account::{GuarantorSigned, Identity},
    value::{Nonce, Value},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct CreditRating(pub GuarantorSigned<CreditRatingPayload>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreditRatingPayload {
    pub nonce: Nonce,
    pub created_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub target: Option<Identity>,
    pub value: Value,
}
