use anyhow::Result;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    metadata::Metadata,
    signature::{Keypair, PublicKey, Signature, SignatureSerializer},
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    <T as Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    <Metadata<T> as Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    <GuaranteeSigned<T> as Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
",))]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct GuarantorSigned<T>
where
    T: Archive,
    <T as Archive>::Archived:
        ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    pub guarantor: Identity,
    pub data: GuaranteeSigned<T>,
}

impl<T> ::core::ops::Deref for GuarantorSigned<T>
where
    T: Archive,
    <T as Archive>::Archived:
        ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    type Target = GuaranteeSigned<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Signer<GuaranteeSigned<T>> for GuarantorSigned<T>
where
    T: ::core::fmt::Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + ::core::hash::Hash
        + Archive
        + Serialize<SignatureSerializer>,
    <T as Archive>::Archived:
        ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    fn sign(account: &Account, data: GuaranteeSigned<T>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(GuarantorSigned {
            guarantor: account.sign(&data)?,
            data,
        })
    }
}

impl<T> Verifier for GuarantorSigned<T>
where
    T: ::core::fmt::Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + ::core::hash::Hash
        + Archive
        + Serialize<SignatureSerializer>,
    <T as Archive>::Archived:
        ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    fn verify(&self) -> Result<()> {
        self.guarantor.verify(&self.data)
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    T: Archive,
    <T as Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    <Metadata<T> as Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
",))]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct GuaranteeSigned<T> {
    pub guarantee: Identity,
    pub data: Metadata<T>,
}

impl<T> ::core::ops::Deref for GuaranteeSigned<T>
where
    T: Archive,
    <T as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    type Target = Metadata<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Signer<Metadata<T>> for GuaranteeSigned<T>
where
    T: Archive + Serialize<SignatureSerializer>,
    <T as Archive>::Archived:
        ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    fn sign(account: &Account, data: Metadata<T>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            guarantee: account.sign(&data)?,
            data,
        })
    }
}

impl<T> Verifier for GuaranteeSigned<T>
where
    T: Archive + Serialize<SignatureSerializer>,
    <T as Archive>::Archived:
        ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    fn verify(&self) -> Result<()> {
        self.guarantee.verify(&self.data)
    }
}

pub trait Signer<T>
where
    T: Serialize<SignatureSerializer>,
{
    fn sign(account: &Account, data: T) -> Result<Self>
    where
        Self: Sized;
}

pub trait Verifier {
    fn verify(&self) -> Result<()>;
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct Identity {
    pub account: AccountRef,
    pub signature: Signature,
}

impl Identity {
    fn verify<T>(&self, data: &T) -> Result<()>
    where
        T: Serialize<SignatureSerializer>,
    {
        use ed25519_dalek::Verifier;

        let data = ::rkyv::to_bytes::<_, 64>(data)?;
        self.account.public_key.verify(&data, &self.signature)?;
        Ok(())
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct AccountRef {
    pub public_key: PublicKey,
}

#[derive(Debug, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Debug))]
pub struct Account {
    keypair: Keypair,
}

impl Account {
    pub(crate) fn sign<T>(&self, data: &T) -> Result<Identity>
    where
        T: Serialize<SignatureSerializer>,
    {
        use ed25519_dalek::Signer;

        Ok(Identity {
            account: AccountRef {
                public_key: PublicKey(self.keypair.public),
            },
            signature: Signature(self.keypair.sign(&::rkyv::to_bytes(data)?)),
        })
    }
}
