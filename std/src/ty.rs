use ipi_core::{generic_array::typenum::U32, value::String};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Type {
    #[serde(flatten)]
    pub leaf: TypeLeaf,
    pub children: Vec<Type>,
}

impl ::core::ops::Deref for Type {
    type Target = TypeLeaf;

    fn deref(&self) -> &Self::Target {
        &self.leaf
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TypeLeaf {
    pub name: String<U32>,
    pub desc: Option<String<U32>>,
}
