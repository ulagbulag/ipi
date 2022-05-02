use bytecheck::CheckBytes;
use num_traits::{FromPrimitive, ToPrimitive};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(archive = "
    <[u8; U] as Archive>::Archived: Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
    <Len as Archive>::Archived: Clone + ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
",))]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct String<const U: usize = 256, Len = u8> {
    pub buf: [u8; U],
    pub len: Len,
}

impl<const U: usize, Len> TryFrom<::std::string::String> for String<U, Len>
where
    Len: FromPrimitive,
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
    Len: ToPrimitive,
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

impl<const U: usize, Len> ::core::fmt::Debug for String<U, Len>
where
    Len: ToPrimitive,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <&str>::try_from(self)
            .map_err(|_| ::core::fmt::Error)
            .and_then(|e| ::core::fmt::Debug::fmt(e, f))
    }
}

impl<const U: usize, Len> ::core::fmt::Display for String<U, Len>
where
    Len: ToPrimitive,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <&str>::try_from(self)
            .map_err(|_| ::core::fmt::Error)
            .and_then(|e| ::core::fmt::Display::fmt(e, f))
    }
}
