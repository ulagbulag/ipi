use ipi_core::value::{text::Text, ValueType};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(serialize = "
    __S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct ClassMetadata {
    pub leaf: ClassLeaf,
    #[omit_bounds]
    pub children: Option<Vec<ClassMetadata>>,
}

impl ::core::ops::Deref for ClassMetadata {
    type Target = ClassLeaf;

    fn deref(&self) -> &Self::Target {
        &self.leaf
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct ClassLeaf {
    pub name: ClassName,
    pub doc: ClassDoc,
    pub ty: ValueType,
}

pub type ClassName = Text;
pub type ClassDoc = Option<Text>;
