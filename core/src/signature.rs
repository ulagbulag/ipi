use rkyv::{ser::serializers::AllocSerializer, Archive, Deserialize, Fallible, Serialize};

pub type SignatureSerializer = AllocSerializer<64>;

#[derive(Copy, Clone, Debug, Eq)]
#[repr(transparent)]
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

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        self.0.as_ref().partial_cmp(other.0.as_ref())
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

#[derive(Copy, Clone, Debug, Eq)]
#[repr(transparent)]
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

impl PartialOrd for PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        self.0.as_ref().partial_cmp(other.0.as_ref())
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

#[derive(Debug)]
#[repr(transparent)]
pub struct Keypair(pub(crate) ::ed25519_dalek::Keypair);

impl ::core::ops::Deref for Keypair {
    type Target = ::ed25519_dalek::Keypair;

    fn deref(&self) -> &Self::Target {
        &self.0
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
