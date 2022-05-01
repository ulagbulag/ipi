#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    <Class as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq,
    <Inner as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, PartialEq))]

pub struct Data<Class = crate::class_data::ClassData, Inner = crate::path::Path> {
    pub class: Class,
    pub inner: Inner,
}

impl<Type, Inner> ::core::ops::Deref for Data<Type, Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
