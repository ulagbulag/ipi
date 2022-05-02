use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{GuarantorSigned, Verifier},
    value::primitives::U64,
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct CreditRating(pub GuarantorSigned<CreditRatingPayload>);

impl ::core::ops::Deref for CreditRating {
    type Target = GuarantorSigned<CreditRatingPayload>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Verifier for CreditRating {
    fn verify(&self) -> anyhow::Result<()> {
        self.0.verify()
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct CreditRatingPayload {
    pub value: U64,
}
