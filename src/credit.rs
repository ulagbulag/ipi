use anyhow::bail;

use crate::{
    account::{GuarantorSigned, Signer, Verifier},
    metadata::Metadata,
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

pub type CreditRatingPayload = Metadata<Value>;

impl Signer<Self> for CreditRatingPayload {
    fn sign(account: &crate::Account, mut data: Self) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        if data.target.is_some() {
            bail!("Already signed");
        }

        data.target.replace(account.sign(&data)?);
        Ok(data)
    }
}
