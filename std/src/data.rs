use crate::{path::Path, ty::Type};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Data {
    pub path: Path,
    #[serde(rename = "type")]
    pub ty: Type,
}
