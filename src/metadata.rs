use anyhow::Result;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{Account, AccountRef, GuaranteeSigned, Signer},
    signature::SignatureSerializer,
    signed::IsSigned,
    value::{chrono::DateTime, hash::Hash, nonce::Nonce},
};

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Archive,
    Serialize,
    Deserialize,
    ::serde::Serialize,
    ::serde::Deserialize,
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

    pub fn build_unsigned_raw(self, guarantor: AccountRef, hash: Hash) -> Metadata {
        Metadata {
            nonce: Nonce::generate(),
            created_date: DateTime::now(),
            expiration_date: self.expiration_date,
            guarantor,
            hash,
        }
    }

    pub fn build_unsigned<T>(self, guarantor: AccountRef, data: &T) -> Result<Metadata>
    where
        T: IsSigned + Archive + Serialize<SignatureSerializer>,
        <T as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    {
        ::rkyv::to_bytes(data)
            .map(|hash| Hash::with_bytes(&hash))
            .map(|hash| self.build_unsigned_raw(guarantor, hash))
            .map_err(Into::into)
    }

    pub fn build<T>(
        self,
        account: &Account,
        guarantor: AccountRef,
        data: &T,
    ) -> Result<GuaranteeSigned>
    where
        T: IsSigned + Archive + Serialize<SignatureSerializer>,
        <T as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    {
        self.build_unsigned(guarantor, data)
            .and_then(|metadata| Signer::sign(account, metadata))
    }
}
