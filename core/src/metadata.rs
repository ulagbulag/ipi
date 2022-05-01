use rkyv::Serialize;

use crate::{
    account::{Identity, Signer},
    signature::SignatureSerializer,
    value::{DateTime, Nonce},
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[repr(C)]
pub struct Metadata<T> {
    pub nonce: Nonce,
    pub created_date: DateTime,
    pub expiration_date: DateTime,
    pub target: Option<Identity>,
    pub data: T,
}

impl<T> ::core::ops::Deref for Metadata<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Signer<Self> for Metadata<T>
where
    T: Serialize<SignatureSerializer>,
{
    fn sign(account: &crate::account::Account, mut data: Self) -> anyhow::Result<Self>
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
