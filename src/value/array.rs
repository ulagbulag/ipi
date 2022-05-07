use anyhow::Result;
use bytecheck::CheckBytes;
use ndarray::{DataOwned, Dimension, RawData};
use rkyv::{option::ArchivedOption, Archive, Deserialize, Fallible, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct Array<A, D>(pub ::ndarray::ArcArray<A, D>)
where
    D: Dimension;

impl<A, D> ::core::ops::Deref for Array<A, D>
where
    D: Dimension,
{
    type Target = ::ndarray::ArcArray<A, D>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A, D> PartialEq<Array<A, D>> for ArrayRaw<A, D>
where
    A: PartialEq,
    D: Dimension,
{
    fn eq(&self, other: &Array<A, D>) -> bool {
        ArchivedOption::Some(self.data.as_slice()) == other.0.as_slice()
            && self.dim == other.0.raw_dim()
    }
}

impl<A, D> Array<A, D>
where
    D: Dimension,
{
    fn try_from_raw(raw: ArrayRaw<A, D>) -> Result<Self>
    where
        A: DataOwned + RawData<Elem = A>,
    {
        ::ndarray::ArrayBase::<A, D>::from_shape_vec(raw.dim, raw.data)
            .map_err(Into::into)
            .map(::ndarray::ArrayBase::into_shared)
            .map(|e| Self(e))
    }

    fn try_to_raw(&self) -> Result<ArrayRaw<A, D>>
    where
        A: Clone + RawData<Elem = A>,
    {
        Ok(ArrayRaw {
            data: self
                .0
                .as_slice()
                .ok_or_else(|| ::anyhow::anyhow!("data is not standard layout"))?
                .to_vec(),
            dim: self.0.raw_dim(),
        })
    }
}

impl<A, D> Archive for Array<A, D>
where
    A: Clone + RawData<Elem = A>,
    Vec<A>: Archive,
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    D: Archive + Dimension,
    <D as Archive>::Archived: ::core::fmt::Debug + PartialEq,
{
    type Archived = <ArrayRaw<A, D> as Archive>::Archived;
    type Resolver = <ArrayRaw<A, D> as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.try_to_raw().unwrap().resolve(pos, resolver, out)
    }
}

impl<A, D, S: Fallible + ?Sized> Serialize<S> for Array<A, D>
where
    A: Clone + RawData<Elem = A>,
    Vec<A>: Archive + Serialize<S>,
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    D: Archive + Serialize<S> + Dimension,
    <D as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.try_to_raw().unwrap().serialize(serializer)
    }
}

impl<A, D, De: Fallible + ?Sized> Deserialize<Array<A, D>, De>
    for <Array<A, D> as Archive>::Archived
where
    A: Clone + DataOwned + RawData<Elem = A>,
    Vec<A>: Archive,
    <Vec<A> as Archive>::Archived: Deserialize<Vec<A>, De> + ::core::fmt::Debug + PartialEq,
    D: Archive + Dimension,
    <D as Archive>::Archived: Deserialize<D, De> + ::core::fmt::Debug + PartialEq,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut De) -> Result<Array<A, D>, De::Error> {
        Deserialize::<ArrayRaw<A, D>, De>::deserialize(self, deserializer)
            // FIXME: handle ndarray errors
            .map(|e| Array::try_from_raw(e).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(bound(archive = "
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    <D as Archive>::Archived: ::core::fmt::Debug + PartialEq,
",))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct ArrayRaw<A, D> {
    data: Vec<A>,
    dim: D,
}
