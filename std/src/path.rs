use ipi_core::value::Hash;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Path {
    pub value: Hash,
    pub len: u64,
}
