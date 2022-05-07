use anyhow::Result;
use bytecheck::CheckBytes;
use ndarray::{DataOwned, Dim, Dimension, Ix, IxDyn};
use rkyv::{Archive, Deserialize, Fallible, Serialize};

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

impl<A, const D: usize> Array<A, Dim<[Ix; D]>>
where
    Dim<[Ix; D]>: Dimension,
{
    fn try_from_raw(raw: ArrayRaw<A, <Dim<[Ix; D]> as Dimension>::Pattern>) -> Result<Self>
    where
        A: DataOwned,
    {
        ::ndarray::Array::<A, Dim<[Ix; D]>>::from_shape_vec(raw.dim, raw.data)
            .map_err(Into::into)
            .map(::ndarray::ArrayBase::into_shared)
            .map(|e| Self(e))
    }

    fn try_to_raw(&self) -> Result<ArrayRaw<A, <Dim<[Ix; D]> as Dimension>::Pattern>>
    where
        A: Clone,
    {
        Ok(ArrayRaw {
            data: self
                .0
                .as_slice()
                .ok_or_else(|| ::anyhow::anyhow!("data is not standard layout"))?
                .to_vec(),
            dim: self.0.dim(),
        })
    }
}

impl<A, const D: usize> Archive for Array<A, Dim<[Ix; D]>>
where
    A: Clone,
    Vec<A>: Archive,
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    Dim<[Ix; D]>: Dimension,
    <Dim<[Ix; D]> as Dimension>::Pattern: Archive,
    <<Dim<[Ix; D]> as Dimension>::Pattern as Archive>::Archived: ::core::fmt::Debug + PartialEq,
{
    type Archived = <ArrayRaw<A, <Dim<[Ix; D]> as Dimension>::Pattern> as Archive>::Archived;
    type Resolver = <ArrayRaw<A, <Dim<[Ix; D]> as Dimension>::Pattern> as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.try_to_raw().unwrap().resolve(pos, resolver, out)
    }
}

impl<A, const D: usize, S: Fallible + ?Sized> Serialize<S> for Array<A, Dim<[Ix; D]>>
where
    A: Clone,
    Vec<A>: Archive + Serialize<S>,
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    Dim<[Ix; D]>: Dimension,
    <Dim<[Ix; D]> as Dimension>::Pattern: Archive + Serialize<S>,
    <<Dim<[Ix; D]> as Dimension>::Pattern as Archive>::Archived: ::core::fmt::Debug + PartialEq,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.try_to_raw().unwrap().serialize(serializer)
    }
}

impl<A, const D: usize, De: Fallible + ?Sized> Deserialize<Array<A, Dim<[Ix; D]>>, De>
    for <Array<A, Dim<[Ix; D]>> as Archive>::Archived
where
    A: Clone + DataOwned,
    Vec<A>: Archive,
    <Vec<A> as Archive>::Archived: Deserialize<Vec<A>, De> + ::core::fmt::Debug + PartialEq,
    Dim<[Ix; D]>: Dimension,
    <Dim<[Ix; D]> as Dimension>::Pattern: Archive,
    <<Dim<[Ix; D]> as Dimension>::Pattern as Archive>::Archived:
        Deserialize<<Dim<[Ix; D]> as Dimension>::Pattern, De> + ::core::fmt::Debug + PartialEq,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut De) -> Result<Array<A, Dim<[Ix; D]>>, De::Error> {
        Deserialize::<ArrayRaw<A, <Dim<[Ix; D]> as Dimension>::Pattern>, De>::deserialize(
            self,
            deserializer,
        )
        // FIXME: handle ndarray errors
        .map(|e| Array::try_from_raw(e).unwrap())
    }
}

impl<A> Array<A, IxDyn> {
    fn try_from_raw_dyn(raw: ArrayRaw<A, Vec<usize>>) -> Result<Self>
    where
        A: DataOwned,
    {
        ::ndarray::Array::<A, IxDyn>::from_shape_vec(raw.dim, raw.data)
            .map_err(Into::into)
            .map(::ndarray::ArrayBase::into_shared)
            .map(|e| Self(e))
    }

    fn try_to_raw_dyn(&self) -> Result<ArrayRaw<A, Vec<usize>>>
    where
        A: Clone,
    {
        Ok(ArrayRaw {
            data: self
                .0
                .as_slice()
                .ok_or_else(|| ::anyhow::anyhow!("data is not standard layout"))?
                .to_vec(),
            dim: self.0.shape().to_vec(),
        })
    }
}

impl<A> Archive for Array<A, IxDyn>
where
    A: Clone,
    Vec<A>: Archive,
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
{
    type Archived = <ArrayRaw<A, Vec<usize>> as Archive>::Archived;
    type Resolver = <ArrayRaw<A, Vec<usize>> as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.try_to_raw_dyn().unwrap().resolve(pos, resolver, out)
    }
}

impl<A, S: Fallible + ?Sized> Serialize<S> for Array<A, IxDyn>
where
    A: Clone,
    Vec<A>: Archive + Serialize<S>,
    <Vec<A> as Archive>::Archived: ::core::fmt::Debug + PartialEq,
    S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.try_to_raw_dyn().unwrap().serialize(serializer)
    }
}

impl<A, De: Fallible + ?Sized> Deserialize<Array<A, IxDyn>, De>
    for <Array<A, IxDyn> as Archive>::Archived
where
    A: Clone + DataOwned,
    Vec<A>: Archive,
    <Vec<A> as Archive>::Archived: Deserialize<Vec<A>, De> + ::core::fmt::Debug + PartialEq,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut De) -> Result<Array<A, IxDyn>, De::Error> {
        Deserialize::<ArrayRaw<A, Vec<usize>>, De>::deserialize(self, deserializer)
            // FIXME: handle ndarray errors
            .map(|e| Array::try_from_raw_dyn(e).unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_rkyv() {
        #[derive(Debug, PartialEq, Archive, Serialize, Deserialize)]
        #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
        pub struct MyScalar {
            data: Array<u8, ndarray::Ix0>,
        }

        #[derive(Debug, PartialEq, Archive, Serialize, Deserialize)]
        #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
        pub struct MyVector {
            data: Array<i32, ndarray::Ix1>,
        }

        #[derive(Debug, PartialEq, Archive, Serialize, Deserialize)]
        #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
        pub struct MyMatrix {
            data: Array<u64, ndarray::Ix2>,
        }

        #[derive(Debug, PartialEq, Archive, Serialize, Deserialize)]
        #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
        pub struct My6Axis {
            data: Array<f32, ndarray::Ix6>,
        }

        #[derive(Debug, PartialEq, Archive, Serialize, Deserialize)]
        #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
        pub struct MyTensor {
            data: Array<f64, ndarray::IxDyn>,
        }
    }
}
