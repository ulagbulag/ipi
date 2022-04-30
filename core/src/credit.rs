use crate::{
    account::{GuarantorSigned, Verifier},
    value::Value,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreditRatingPayload {
    pub value: Value,
}
