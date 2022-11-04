use std::borrow::Cow;

use cid::{
    multihash::{Code, MultihashDigest},
    Cid,
};
use quick_protobuf::{MessageWrite, Writer};
use rkyv::{Archive, Deserialize, Fallible, Serialize};

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Deserialize,
)]
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

impl TryFrom<&Hash> for [u8; Hash::SIZE] {
    type Error = ::cid::Error;

    fn try_from(hash: &Hash) -> Result<Self, Self::Error> {
        hash.0
            .into_v1()
            .map(|cid| cid.to_bytes().as_slice().try_into().unwrap())
    }
}

impl Into<Vec<u8>> for Hash {
    fn into(self) -> Vec<u8> {
        self.0.to_bytes()
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
        // FIXME: handle Cid parsing errors
        <[u8; Hash::SIZE]>::try_from(self)
            .unwrap()
            .resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Hash
where
    S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        // FIXME: handle Cid parsing errors
        <[u8; Hash::SIZE]>::try_from(self)
            .unwrap()
            .serialize(serializer)
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

impl ::serde::Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        ::serde::Serialize::serialize(&self.to_string(), serializer)
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
            links: {
                #[cfg(target_os = "wasi")]
                {
                    bytes
                        .chunks(chunk_size)
                        .map(|chunk| Self::calculate_link(&chunk, sublevel))
                        .collect()
                }

                #[cfg(not(target_os = "wasi"))]
                {
                    if bytes.len() == Self::MAX_LINKS * chunk_size {
                        use rayon::prelude::*;

                        bytes
                            .to_vec()
                            .into_par_iter()
                            .chunks(chunk_size)
                            .map(|chunk| Self::calculate_link(&chunk, sublevel))
                            .collect()
                    } else {
                        bytes
                            .chunks(chunk_size)
                            .map(|chunk| Self::calculate_link(&chunk, sublevel))
                            .collect()
                    }
                }
            },
        };

        // compute CID
        Self::with_bytes_dag_raw(&node)
    }

    fn calculate_link(chunk: &[u8], sublevel: u32) -> ::unixfs::PBLink<'static> {
        let chunk = chunk.as_ref();
        let (hash, dag_size) = Self::with_bytes_dag(chunk, sublevel);

        ::unixfs::PBLink {
            Hash: Some(hash.0.to_bytes().into()),
            Name: Some(Default::default()),
            Tsize: Some(chunk.len() as u64 + dag_size),
        }
    }

    fn with_bytes_dag_raw(node: &::unixfs::FlatUnixFs) -> (Self, u64) {
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

#[derive(Debug, Default)]
pub struct Hasher {
    buf: Vec<u8>,
    len: usize,
    nodes: Vec<::unixfs::FlatUnixFs<'static>>,
}

impl Hasher {
    fn push(&mut self, sublevel: usize, chunk_size: u64, hash: Hash, dag_size: u64) {
        // reserve a slot in the tree
        if sublevel == 0 {
            for sublevel in 0..self.nodes.len() {
                let node = self.nodes.get(sublevel).unwrap();

                // flush the node
                if node.links.len() == Hash::MAX_LINKS {
                    // read hash digest
                    let chunk_size = node.data.filesize.unwrap();
                    let (hash, dag_size) = Hash::with_bytes_dag_raw(&node);

                    // update parent
                    self.push(sublevel + 1, chunk_size, hash, dag_size);

                    let mut node = self.nodes.get_mut(sublevel).unwrap();

                    // update the UnixFS Data
                    node.data.blocksizes.clear();
                    node.data.filesize = Some(0);

                    // update the Links
                    node.links.clear();
                } else {
                    break;
                }
            }
        }

        let node = match self.nodes.get_mut(sublevel) {
            Some(node) => node,
            None => {
                // note: skipping 2 or more levels are illegal!
                self.nodes.push(::unixfs::FlatUnixFs {
                    data: ::unixfs::UnixFs {
                        Type: ::unixfs::UnixFsType::File,
                        filesize: Some(0),
                        ..Default::default()
                    },
                    links: Default::default(),
                });
                self.nodes.get_mut(sublevel).unwrap()
            }
        };

        // update the UnixFS Data
        node.data.blocksizes.push(chunk_size);
        *node.data.filesize.as_mut().unwrap() += chunk_size;

        // update the Links
        node.links.push(::unixfs::PBLink {
            Hash: Some(hash.0.to_bytes().into()),
            Name: Some(Default::default()),
            Tsize: Some(chunk_size + dag_size),
        });
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn update(&mut self, mut bytes: &[u8]) {
        self.len += bytes.len();

        // add full chunks
        while self.buf.len() + bytes.len() > Hash::CHUNK_SIZE {
            // get chunk buffer
            let chunk: Cow<[u8]> = if self.buf.is_empty() {
                let buf = &bytes[..Hash::CHUNK_SIZE];
                bytes = &bytes[Hash::CHUNK_SIZE..];
                buf.into()
            } else {
                let bytes_len = Hash::CHUNK_SIZE - self.buf.len();

                let buf = [&self.buf, &bytes[..bytes_len]].concat();
                self.buf.clear();
                bytes = &bytes[bytes_len..];
                buf.into()
            };

            // insert to the leaf node
            {
                // read hash digest
                let chunk_size = chunk.len() as u64;
                let (hash, dag_size) = Hash::with_bytes_chunk(&chunk);

                // get or create the leaf node
                self.push(0, chunk_size, hash, dag_size);
            }
        }

        // retain the unfulfilled chunk
        self.buf.extend_from_slice(bytes);
    }

    pub fn finalize(mut self) -> Hash {
        // if there is no DAG, then return the raw chunk's hash
        if self.nodes.is_empty() {
            // read hash digest
            let (hash, _) = Hash::with_bytes_chunk(&self.buf);

            // compose CID
            return hash;
        }

        // insert the unfulfilled chunk
        if !self.buf.is_empty() {
            // read hash digest
            let chunk_size = self.buf.len() as u64;
            let (hash, dag_size) = Hash::with_bytes_chunk(&self.buf);

            // get or create the leaf node
            self.push(0, chunk_size, hash, dag_size);
        }

        // insert all subnodes
        for sublevel in 0..self.nodes.len() - 1 {
            let node = self.nodes.get(sublevel).unwrap();

            // read hash digest
            let chunk_size = node.data.filesize.unwrap();
            let (hash, dag_size) = Hash::with_bytes_dag_raw(&node);

            // update parent
            self.push(sublevel + 1, chunk_size, hash, dag_size);
        }

        // read hash digest
        let (hash, _) = Hash::with_bytes_dag_raw(&self.nodes.last().unwrap());

        // compose CID
        hash
    }
}
