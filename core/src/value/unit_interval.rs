use fixed::{traits::ToFixed, types::U0F32};
use rkyv::{
    rend::{BigEndian, LittleEndian},
    Archive, Deserialize, Fallible, Serialize,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitInterval(pub U0F32);

impl<Src> From<Src> for UnitInterval
where
    Src: ToFixed,
{
    fn from(value: Src) -> Self {
        Self(value.checked_to_fixed().unwrap_or_default())
    }
}

impl ::core::ops::Deref for UnitInterval {
    type Target = U0F32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<UnitInterval> for BigEndian<u32> {
    fn eq(&self, other: &UnitInterval) -> bool {
        self == &other.0.to_bits()
    }
}

impl PartialEq<UnitInterval> for LittleEndian<u32> {
    fn eq(&self, other: &UnitInterval) -> bool {
        self == &other.0.to_bits()
    }
}

impl PartialOrd<UnitInterval> for BigEndian<u32> {
    fn partial_cmp(&self, other: &UnitInterval) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&other.0.to_bits())
    }
}

impl PartialOrd<UnitInterval> for LittleEndian<u32> {
    fn partial_cmp(&self, other: &UnitInterval) -> Option<::core::cmp::Ordering> {
        self.partial_cmp(&other.0.to_bits())
    }
}

impl Archive for UnitInterval {
    type Archived = <u32 as Archive>::Archived;
    type Resolver = <u32 as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.to_bits().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for UnitInterval {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.to_bits().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<UnitInterval, D> for <UnitInterval as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<UnitInterval, D::Error> {
        Deserialize::<u32, D>::deserialize(self, deserializer)
            .map(U0F32::from_bits)
            .map(UnitInterval)
    }
}
