use ipi_core::value::{text::Text, ValueType};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(serialize = "
    __S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct ClassMetadata {
    pub leaf: ClassLeaf,
    #[omit_bounds]
    pub children: Vec<ClassMetadata>,
}

impl ::core::ops::Deref for ClassMetadata {
    type Target = ClassLeaf;

    fn deref(&self) -> &Self::Target {
        &self.leaf
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct ClassLeaf {
    pub name: ClassName,
    pub doc: ClassDoc,
    pub ty: ValueType,
}

impl ::core::fmt::Display for ClassLeaf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.name, f)
    }
}

pub type ClassName = Text<32>;
pub type ClassDoc = Text<256>;
