use anyhow::anyhow;
use base58::{FromBase58, ToBase58};
use rkyv::{ser::serializers::AllocSerializer, Archive, Deserialize, Fallible, Serialize};

pub type SignatureSerializer = AllocSerializer<64>;

#[derive(Copy, Clone, Debug, Eq, ::serde::Serialize, ::serde::Deserialize)]
pub struct Signature(pub(crate) ::ed25519_dalek::Signature);

impl ::core::ops::Deref for Signature {
    type Target = ::ed25519_dalek::Signature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Signature> for [u8; 64] {
    fn eq(&self, other: &Signature) -> bool {
        self == other.0.as_ref()
    }
}

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        self.0.as_ref().partial_cmp(other.0.as_ref())
    }
}

impl PartialOrd<Signature> for [u8; 64] {
    fn partial_cmp(&self, other: &Signature) -> Option<::core::cmp::Ordering> {
        self.as_ref().partial_cmp(other.0.as_ref())
    }
}

impl Ord for Signature {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.0.as_ref().cmp(other.0.as_ref())
    }
}

impl ::core::hash::Hash for Signature {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().hash(state);
    }
}

impl ::core::str::FromStr for Signature {
    type Err = ::anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .from_base58()
            .map_err(|_| anyhow!("failed to parse Signature"))?;
        Ok(Self(::ed25519_dalek::Signature::from_bytes(&bytes)?))
    }
}

impl ToString for Signature {
    fn to_string(&self) -> String {
        self.0.as_ref().to_base58()
    }
}

impl Archive for Signature {
    type Archived = <[u8; 64] as Archive>::Archived;
    type Resolver = <[u8; 64] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.to_bytes().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Signature {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.to_bytes().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Signature, D> for <Signature as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Signature, D::Error> {
        Deserialize::<[u8; 64], D>::deserialize(self, deserializer)
            // FIXME: handle signature errors
            .map(|ref e| ::ed25519_dalek::Signature::from_bytes(e).unwrap())
            .map(Signature)
    }
}

#[derive(Copy, Clone, Debug, Eq, ::serde::Serialize, ::serde::Deserialize)]
pub struct PublicKey(pub(crate) ::ed25519_dalek::PublicKey);

impl ::core::ops::Deref for PublicKey {
    type Target = ::ed25519_dalek::PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<PublicKey> for [u8; 32] {
    fn eq(&self, other: &PublicKey) -> bool {
        self == other.0.as_ref()
    }
}

impl PartialOrd for PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        self.0.as_ref().partial_cmp(other.0.as_ref())
    }
}

impl PartialOrd<PublicKey> for [u8; 32] {
    fn partial_cmp(&self, other: &PublicKey) -> Option<::core::cmp::Ordering> {
        self.as_ref().partial_cmp(other.0.as_ref())
    }
}

impl Ord for PublicKey {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.0.as_ref().cmp(other.0.as_ref())
    }
}

impl ::core::hash::Hash for PublicKey {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().hash(state);
    }
}

impl ::core::str::FromStr for PublicKey {
    type Err = ::anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .from_base58()
            .map_err(|_| anyhow!("failed to parse PublicKey"))?;
        Ok(Self(::ed25519_dalek::PublicKey::from_bytes(&bytes)?))
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        self.0.as_ref().to_base58()
    }
}

impl Archive for PublicKey {
    type Archived = <[u8; 32] as Archive>::Archived;
    type Resolver = <[u8; 32] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.to_bytes().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for PublicKey {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.to_bytes().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<PublicKey, D> for <PublicKey as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<PublicKey, D::Error> {
        Deserialize::<[u8; 32], D>::deserialize(self, deserializer)
            // FIXME: handle signature errors
            .map(|ref e| ::ed25519_dalek::PublicKey::from_bytes(e).unwrap())
            .map(PublicKey)
    }
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
pub struct Keypair(pub(crate) ::ed25519_dalek::Keypair);

impl ::core::ops::Deref for Keypair {
    type Target = ::ed25519_dalek::Keypair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<Keypair> for [u8; 64] {
    fn eq(&self, other: &Keypair) -> bool {
        self == &other.0.to_bytes()
    }
}

impl PartialOrd<Keypair> for [u8; 64] {
    fn partial_cmp(&self, other: &Keypair) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&other.0.to_bytes())
    }
}

impl ::core::str::FromStr for Keypair {
    type Err = ::anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .from_base58()
            .map_err(|_| anyhow!("failed to parse Keypair"))?;
        Ok(Self(::ed25519_dalek::Keypair::from_bytes(&bytes)?))
    }
}

impl ToString for Keypair {
    fn to_string(&self) -> String {
        self.0.to_bytes().to_base58()
    }
}

impl Archive for Keypair {
    type Archived = <[u8; 64] as Archive>::Archived;
    type Resolver = <[u8; 64] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.to_bytes().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Keypair {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.to_bytes().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Keypair, D> for <Keypair as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Keypair, D::Error> {
        Deserialize::<[u8; 64], D>::deserialize(self, deserializer)
            // FIXME: handle signature errors
            .map(|ref e| ::ed25519_dalek::Keypair::from_bytes(e).unwrap())
            .map(Keypair)
    }
}

impl Keypair {
    pub fn generate() -> Self {
        Self(::ed25519_dalek::Keypair::generate(
            &mut ::rand::rngs::OsRng {},
        ))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public)
    }

    pub fn secret_key(&self) -> &::ed25519_dalek::SecretKey {
        &self.0.secret
    }
}
