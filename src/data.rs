use std::marker::PhantomData;

use anyhow::Result;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    account::{Account, AccountRef, GuaranteeSigned, GuarantorSigned, Signer, Verifier},
    metadata::{Metadata, MetadataBuilder},
    signature::SignatureSerializer,
    signed::IsSigned,
    value::chrono::DateTime,
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    Metadata: Archive,
    <Metadata as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    RawData: Archive,
    <RawData as Archive>::Archived: ::core::fmt::Debug + PartialEq,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct Data<Metadata, RawData>
where
    Metadata: Verifier,
    RawData: IsSigned,
{
    pub metadata: Metadata,
    pub data: RawData,
}

impl<Metadata, RawData> ::core::ops::Deref for Data<Metadata, RawData>
where
    Metadata: Verifier,
    RawData: IsSigned,
{
    type Target = RawData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Metadata, RawData> Verifier for Data<Metadata, RawData>
where
    Metadata: Verifier,
    RawData: IsSigned,
{
    fn verify(&self, guarantor: Option<&AccountRef>) -> Result<()> {
        // skip validation of raw data
        self.metadata.verify(guarantor)
    }
}

impl<RawData> Data<GuaranteeSigned, RawData>
where
    RawData: IsSigned,
{
    pub fn builder() -> DataBuilder<RawData> {
        DataBuilder {
            metadata: Metadata::builder(),
            _data: Default::default(),
        }
    }

    pub fn sign(self, guarantor: &Account) -> Result<Data<GuarantorSigned, RawData>> {
        Ok(Data {
            metadata: GuarantorSigned::sign(guarantor, self.metadata)?,
            data: self.data,
        })
    }
}

impl<'a, Metadata, RawData> Data<Metadata, &'a RawData>
where
    Metadata: Copy + Verifier,
    RawData: Clone + IsSigned,
{
    pub fn to_owned(&self) -> Data<Metadata, RawData> {
        Data {
            metadata: self.metadata,
            data: self.data.clone(),
        }
    }
}

pub struct DataBuilder<T> {
    metadata: MetadataBuilder,
    _data: PhantomData<T>,
}

impl<T> DataBuilder<T> {
    pub fn expiration_date(mut self, date: DateTime) -> Self {
        self.metadata = self.metadata.expiration_date(date);
        self
    }

    pub fn build<'a>(
        self,
        account: &Account,
        guarantor: AccountRef,
        data: &'a T,
    ) -> Result<Data<GuaranteeSigned, &'a T>>
    where
        T: IsSigned + Archive + Serialize<SignatureSerializer>,
        <T as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    {
        let metadata = self.metadata.build(account, guarantor, data)?;

        Ok(Data { metadata, data })
    }

    pub fn build_owned(
        self,
        account: &Account,
        guarantor: AccountRef,
        data: T,
    ) -> Result<Data<GuaranteeSigned, T>>
    where
        T: IsSigned + Archive + Serialize<SignatureSerializer>,
        <T as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    {
        Ok(Data {
            metadata: self.build(account, guarantor, &data)?.metadata,
            data,
        })
    }
}
