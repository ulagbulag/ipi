use ipi_core::value::string::String;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(serialize = "
    __S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, PartialEq))]
pub struct ClassData {
    pub leaf: ClassLeaf,
    #[omit_bounds]
    pub children: Vec<ClassData>,
}

impl ::core::ops::Deref for ClassData {
    type Target = ClassLeaf;

    fn deref(&self) -> &Self::Target {
        &self.leaf
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct ClassLeaf {
    pub name: ClassName,
    pub doc: ClassDoc,
}

pub type ClassName = String<32>;
pub type ClassDoc = String<256>;
