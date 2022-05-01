use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{Identity, Signer},
    signature::SignatureSerializer,
    value::{chrono::DateTime, nonce::Nonce},
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    T: Archive,
    <T as Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
"))]
#[archive_attr(derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
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
    T: Archive + Serialize<SignatureSerializer>,
    <T as Archive>::Archived:
        Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
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
