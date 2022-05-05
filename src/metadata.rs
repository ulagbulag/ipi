use std::marker::PhantomData;

use anyhow::Result;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{Account, AccountRef, GuaranteeSigned, Signer},
    signature::SignatureSerializer,
    value::{chrono::DateTime, nonce::Nonce},
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    <T as Archive>::Archived: ::core::fmt::Debug + PartialEq,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct Metadata<T> {
    pub nonce: Nonce,
    pub created_date: DateTime,
    pub expiration_date: Option<DateTime>,
    pub guarantor: AccountRef,
    pub data: T,
}

impl<T> ::core::ops::Deref for Metadata<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Metadata<T> {
    pub fn builder() -> MetadataBuilder<T> {
        MetadataBuilder {
            expiration_date: None,
            _data: Default::default(),
        }
    }
}

pub struct MetadataBuilder<T> {
    expiration_date: Option<DateTime>,
    _data: PhantomData<T>,
}

impl<T> MetadataBuilder<T> {
    pub fn expiration_date(mut self, date: DateTime) -> Self {
        self.expiration_date = Some(date);
        self
    }

    pub fn build(
        self,
        account: &Account,
        guarantor: AccountRef,
        data: T,
    ) -> Result<GuaranteeSigned<T>>
    where
        T: Archive + Serialize<SignatureSerializer>,
        <T as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    {
        let metadata = Metadata {
            nonce: Nonce::generate(),
            created_date: DateTime::now(),
            expiration_date: self.expiration_date,
            guarantor,
            data,
        };

        Signer::sign(account, metadata)
    }
}
