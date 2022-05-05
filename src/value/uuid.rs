use rkyv::{
    rend::{BigEndian, LittleEndian},
    Archive, Deserialize, Fallible, Serialize,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid(pub ::uuid::Uuid);

impl ::core::ops::Deref for Uuid {
    type Target = ::uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<Uuid> for BigEndian<u128> {
    fn eq(&self, other: &Uuid) -> bool {
        self == &other.0.as_u128()
    }
}

impl PartialEq<Uuid> for LittleEndian<u128> {
    fn eq(&self, other: &Uuid) -> bool {
        self == &other.0.as_u128()
    }
}

impl PartialOrd<Uuid> for BigEndian<u128> {
    fn partial_cmp(&self, other: &Uuid) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&other.0.as_u128())
    }
}

impl PartialOrd<Uuid> for LittleEndian<u128> {
    fn partial_cmp(&self, other: &Uuid) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&other.0.as_u128())
    }
}

impl Archive for Uuid {
    type Archived = <u128 as Archive>::Archived;
    type Resolver = <u128 as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.as_u128().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Uuid {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.as_u128().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Uuid, D> for <Uuid as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Uuid, D::Error> {
        Deserialize::<u128, D>::deserialize(self, deserializer)
            .map(::uuid::Uuid::from_u128)
            .map(Uuid)
    }
}

impl Uuid {
    pub fn generate() -> Self {
        Self(::uuid::Uuid::new_v4())
    }
}
