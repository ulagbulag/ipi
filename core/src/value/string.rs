use num_traits::{FromPrimitive, ToPrimitive};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct String<const U: usize = 256, Len = u8>
where
    <[u8; U] as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    Len: Archive,
    <Len as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    pub buf: [u8; U],
    pub len: Len,
}

impl<const U: usize, Len> TryFrom<::std::string::String> for String<U, Len>
where
    <[u8; U] as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    Len: Archive + FromPrimitive,
    <Len as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    type Error = ::anyhow::Error;

    fn try_from(value: ::std::string::String) -> Result<Self, Self::Error> {
        let buf: [u8; U] = value
            .into_bytes()
            .try_into()
            .or_else(|_| bail!("Buffer Overflow"))?;
        let len = Len::from_usize(buf.len()).ok_or_else(|| anyhow!("Buffer Overflow"))?;

        Ok(Self { buf, len })
    }
}

impl<'a, const U: usize, Len> TryFrom<&'a String<U, Len>> for &'a str
where
    <[u8; U] as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    Len: Archive + ToPrimitive,
    <Len as Archive>::Archived:
        Copy + Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
{
    type Error = ::anyhow::Error;

    fn try_from(value: &'a String<U, Len>) -> Result<Self, Self::Error> {
        ::core::str::from_utf8(
            &value.buf[..value
                .len
                .to_usize()
                .ok_or_else(|| anyhow!("Buffer Overflow"))?],
        )
        .map_err(Into::into)
    }
}
