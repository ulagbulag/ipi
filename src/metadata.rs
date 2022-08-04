use anyhow::Result;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{Account, AccountRef, GuaranteeSigned, Signer},
    value::{chrono::DateTime, hash::Hash, nonce::Nonce},
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct Metadata {
    pub nonce: Nonce,
    pub created_date: DateTime,
    pub expiration_date: Option<DateTime>,
    pub guarantor: AccountRef,
    pub hash: Hash,
}

impl Metadata {
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder {
            expiration_date: None,
        }
    }
}

pub struct MetadataBuilder {
    expiration_date: Option<DateTime>,
}

impl MetadataBuilder {
    pub fn expiration_date(mut self, date: DateTime) -> Self {
        self.expiration_date = Some(date);
        self
    }

    pub fn build_unsigned(self, guarantor: AccountRef, hash: Hash) -> Metadata {
        Metadata {
            nonce: Nonce::generate(),
            created_date: DateTime::now(),
            expiration_date: self.expiration_date,
            guarantor,
            hash,
        }
    }

    pub fn build(
        self,
        account: &Account,
        guarantor: AccountRef,
        hash: Hash,
    ) -> Result<GuaranteeSigned> {
        Signer::sign(account, self.build_unsigned(guarantor, hash))
    }
}
