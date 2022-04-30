use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    account::{Identity, Signer},
    value::Nonce,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct Metadata<T> {
    pub nonce: Nonce,
    pub created_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
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
    T: Serialize,
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
