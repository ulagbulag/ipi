use ipi_core::value::UnitInterval;

use crate::data::Data;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Attention {
    pub attention: UnitInterval,
    pub confidence: UnitInterval,
    pub data: Data,
}
