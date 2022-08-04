use anyhow::Result;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{AccountRef, GuarantorSigned, Verifier},
    data::Data,
    value::primitives::U64,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct CreditRating(pub Data<GuarantorSigned, CreditRatingPayload>);

impl ::core::ops::Deref for CreditRating {
    type Target = Data<GuarantorSigned, CreditRatingPayload>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Verifier for CreditRating {
    fn verify(&self, guarantor: Option<&AccountRef>) -> Result<()> {
        self.0.verify(guarantor)
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
