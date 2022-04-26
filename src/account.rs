use core::ops::{Deref, DerefMut};

use anyhow::Result;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuarantorSigned<T> {
    pub guarantor: Identity,
    pub data: GuaranteeSigned<T>,
}

impl<T> Deref for GuarantorSigned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for GuarantorSigned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuaranteeSigned<T> {
    pub guarantee: Identity,
    pub data: T,
}

impl<T> Deref for GuaranteeSigned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for GuaranteeSigned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Identity {
    pub public_key: PublicKey,
    pub signature: Signature,
}

impl PartialEq for Identity {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key && self.signature == other.signature
    }
}

impl PartialOrd for Identity {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        match self
            .public_key
            .as_ref()
            .partial_cmp(other.public_key.as_ref())
        {
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
        self.public_key
            .as_ref()
            .cmp(other.public_key.as_ref())
            .then(self.signature.as_ref().cmp(other.signature.as_ref()))
    }
}

impl Identity {
    fn sign<T>(keypair: &Keypair, data: &T) -> Result<Self>
    where
        T: ::serde::Serialize,
    {
        Ok(Self {
            public_key: keypair.public,
            signature: keypair.sign(&::bincode::serialize(data)?),
        })
    }
}

impl ::core::hash::Hash for Identity {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.public_key.as_ref().hash(state);
        self.signature.as_ref().hash(state);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub keypair: Keypair,
}

impl Account {
    pub fn sign_guarantee<T>(&self, data: T) -> Result<GuaranteeSigned<T>>
    where
        T: ::serde::Serialize,
    {
        Ok(GuaranteeSigned {
            guarantee: Identity::sign(&self.keypair, &data)?,
            data,
        })
    }

    pub fn sign_as_guarantor<T>(&self, data: GuaranteeSigned<T>) -> Result<GuarantorSigned<T>>
    where
        T: ::serde::Serialize,
    {
        Ok(GuarantorSigned {
            guarantor: Identity::sign(&self.keypair, &data)?,
            data,
        })
    }
}
