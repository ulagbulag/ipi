#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct Attention<Unit = AttentionUnit, Data = crate::data::Data> {
    pub attention: Unit,
    pub confidence: Unit,
    pub data: Data,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum AttentionUnit {
    Always,
    Virtually,
    Usually,
    Sometimes,
    Ever,
    Never,
}
