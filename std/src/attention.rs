#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    <Unit as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq,
    <Data as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct Attention<Unit = AttentionUnit, Data = crate::data::Data> {
    pub attention: Unit,
    pub confidence: Unit,
    pub data: Data,
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub enum AttentionUnit {
    Always,
    Virtually,
    Usually,
    Sometimes,
    Ever,
    Never,
}
