use generic_array::GenericArray;
use rkyv::{Archive, Deserialize, Fallible, Serialize};
use sha2::{digest::OutputSizeUser, Sha256VarCore};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash(pub GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize>);

impl ::core::ops::Deref for Hash {
    type Target = GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<Hash> for [u8; 32] {
    fn eq(&self, other: &Hash) -> bool {
        self == &Self::from(other.0)
    }
}

impl PartialOrd<Hash> for [u8; 32] {
    fn partial_cmp(&self, other: &Hash) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&Self::from(other.0))
    }
}

impl Archive for Hash {
    type Archived = <[u8; 32] as Archive>::Archived;
    type Resolver = <[u8; 32] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        <[u8; 32]>::from(self.0).resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Hash {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        <[u8; 32]>::from(self.0).serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Hash, D> for <Hash as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Hash, D::Error> {
        Deserialize::<[u8; 32], D>::deserialize(self, deserializer)
            .map(Into::into)
            .map(Hash)
    }
}
