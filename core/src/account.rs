use anyhow::Result;
use rkyv::Serialize;

use crate::{
    metadata::Metadata,
    signature::{Keypair, PublicKey, Signature, SignatureSerializer},
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[repr(C)]
pub struct GuarantorSigned<T> {
    pub guarantor: Identity,
    pub data: GuaranteeSigned<T>,
}

impl<T> ::core::ops::Deref for GuarantorSigned<T> {
    type Target = GuaranteeSigned<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Signer<GuaranteeSigned<T>> for GuarantorSigned<T>
where
    T: Serialize<SignatureSerializer>,
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
    T: Serialize<SignatureSerializer>,
{
    fn verify(&self) -> Result<()> {
        self.guarantor.verify(&self.data)
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[repr(C)]
pub struct GuaranteeSigned<T> {
    pub guarantee: Identity,
    pub data: Metadata<T>,
}

impl<T> ::core::ops::Deref for GuaranteeSigned<T> {
    type Target = Metadata<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Signer<Metadata<T>> for GuaranteeSigned<T>
where
    T: Serialize<SignatureSerializer>,
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
    T: Serialize<SignatureSerializer>,
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

#[derive(Copy, Clone, Debug, Eq, Archive, Serialize, Deserialize)]
#[repr(C)]
pub struct Identity {
    pub account: AccountRef,
    pub signature: Signature,
}

impl PartialEq for Identity {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account && self.signature == other.signature
    }
}

impl PartialOrd for Identity {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        match self.account.partial_cmp(&other.account) {
            Some(::core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.signature
            .as_ref()
            .partial_cmp(other.signature.as_ref())
    }
}

impl Ord for Identity {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.account
            .cmp(&other.account)
            .then(self.signature.as_ref().cmp(other.signature.as_ref()))
    }
}

impl ::core::hash::Hash for Identity {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.account.hash(state);
        self.signature.as_ref().hash(state);
    }
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

#[derive(Copy, Clone, Debug, Eq, Archive, Serialize, Deserialize)]
#[repr(C)]
pub struct AccountRef {
    pub public_key: PublicKey,
}

impl PartialEq for AccountRef {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key
    }
}

impl PartialOrd for AccountRef {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        self.public_key
            .as_ref()
            .partial_cmp(other.public_key.as_ref())
    }
}

impl Ord for AccountRef {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.public_key.as_ref().cmp(other.public_key.as_ref())
    }
}

impl ::core::hash::Hash for AccountRef {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.public_key.as_ref().hash(state);
    }
}

#[derive(Debug)]
#[repr(C)]
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
