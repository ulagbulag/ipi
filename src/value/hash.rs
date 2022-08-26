use cid::{
    multihash::{Code, MultihashDigest},
    Cid,
};
use quick_protobuf::{MessageWrite, Writer};
use rkyv::{Archive, Deserialize, Fallible, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash(Cid);

impl PartialEq<Hash> for [u8; Hash::SIZE] {
    fn eq(&self, other: &Hash) -> bool {
        self.as_slice() == other.0.to_bytes()
    }
}

impl PartialOrd<Hash> for [u8; Hash::SIZE] {
    fn partial_cmp(&self, other: &Hash) -> Option<::core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.0.to_bytes().as_slice())
    }
}

impl ::core::str::FromStr for Hash {
    type Err = ::anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cid::from_str(s).map(Self).map_err(Into::into)
    }
}

impl From<&Hash> for [u8; Hash::SIZE] {
    fn from(hash: &Hash) -> Self {
        hash.0.to_bytes().as_slice().try_into().unwrap()
    }
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Archive for Hash {
    type Archived = <[u8; Hash::SIZE] as Archive>::Archived;
    type Resolver = <[u8; Hash::SIZE] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        <[u8; Hash::SIZE]>::from(self).resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Hash
where
    S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        <[u8; Hash::SIZE]>::from(self).serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Hash, D> for <Hash as Archive>::Archived {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Hash, D::Error> {
        Deserialize::<[u8; Hash::SIZE], D>::deserialize(self, deserializer)
            // FIXME: handle Cid parsing errors
            .map(|bytes| Cid::try_from(bytes.as_slice()).unwrap())
            .map(Hash)
    }
}

impl Hash {
    /// should be matched with IPFS's chunk size
    const CHUNK_SIZE: usize = 262_144;

    /// should be matched with IPFS's max links
    const MAX_LINKS: usize = 174;

    /// Raw multicodec code
    const CODEC_RAW: u64 = 0x55;

    /// DAG-PB multicodec code
    const CODEC_DAG_PB: u64 = 0x70;

    /// Hash size
    const SIZE: usize = 32 + 4;

    pub fn with_bytes(bytes: &[u8]) -> Self {
        let level = {
            let num_bytes = bytes.len();
            if num_bytes <= Self::CHUNK_SIZE {
                0
            } else {
                let num_chunks = (num_bytes - 1) / Self::CHUNK_SIZE + 1;

                let mut level = 1;
                let mut max_chunks_per_level = Self::MAX_LINKS;

                while num_chunks > max_chunks_per_level {
                    level += 1;
                    max_chunks_per_level *= Self::MAX_LINKS;
                }
                level
            }
        };

        Self::with_bytes_dag(bytes, level).0
    }

    fn with_bytes_dag(bytes: &[u8], level: u32) -> (Self, u64) {
        // solve unit chunks
        if level == 0 {
            return Self::with_bytes_chunk(bytes);
        }

        let sublevel = level - 1;
        let chunk_size = Self::CHUNK_SIZE * Self::MAX_LINKS.pow(sublevel);

        // compose DAG
        let node = ::unixfs::FlatUnixFs {
            data: ::unixfs::UnixFs {
                Type: ::unixfs::UnixFsType::File,
                filesize: Some(bytes.len().try_into().expect("Too large data size")),
                blocksizes: bytes
                    .chunks(chunk_size)
                    .map(|chunk| chunk.len() as u64)
                    .collect(),
                ..Default::default()
            },
            links: bytes
                .chunks(chunk_size)
                .map(|chunk| {
                    let (hash, size) = Self::with_bytes_dag(chunk, sublevel);

                    ::unixfs::PBLink {
                        Hash: Some(hash.0.to_bytes().into()),
                        Name: Some(Default::default()),
                        Tsize: Some(chunk.len() as u64 + size),
                    }
                })
                .collect(),
        };

        // read hash digest
        let buf = {
            let mut buf = Vec::new();
            let mut writer = Writer::new(&mut buf);
            node.write_message(&mut writer)
                .expect("Failed to write DAG");
            buf
        };
        let hash = Code::Sha2_256.digest(&buf);

        // compose CID
        (
            Self(Cid::new_v1(Self::CODEC_DAG_PB, hash)),
            buf.len() as u64,
        )
    }

    fn with_bytes_chunk(bytes: &[u8]) -> (Self, u64) {
        let num_bytes = bytes.len();

        // assert chunk size
        debug_assert!(num_bytes <= Self::CHUNK_SIZE);

        // read hash digest
        let hash = Code::Sha2_256.digest(bytes);

        // compose CID
        (Self(Cid::new_v1(Self::CODEC_RAW, hash)), 0)
    }

    pub fn with_str(msg: &str) -> Self {
        Self::with_bytes(msg.as_bytes())
    }
}
