pub mod primitives;

use std::borrow::Cow;

pub trait Object {
    type Cursor: Clone + ::core::fmt::Debug + Default;

    fn __object_name(&self) -> Cow<crate::class::metadata::ClassName>;

    fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc>;

    fn __object_value_ty(&self) -> ::ipi_core::value::ValueType;

    fn __object_children(&self) -> Option<Cow<[crate::class::metadata::ClassMetadata]>>;

    fn __object_metadata(&self) -> Cow<crate::class::metadata::ClassMetadata>;

    fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf>;

    fn cursor(&self) -> <Self as Object>::Cursor {
        <<Self as Object>::Cursor as Default>::default()
    }
}

impl<T> Object for &T
where
    T: Object,
{
    type Cursor = <T as Object>::Cursor;

    fn __object_name(&self) -> Cow<crate::class::metadata::ClassName> {
        <T as Object>::__object_name(*self)
    }

    fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc> {
        <T as Object>::__object_doc(*self)
    }

    fn __object_value_ty(&self) -> ::ipi_core::value::ValueType {
        <T as Object>::__object_value_ty(*self)
    }

    fn __object_children(&self) -> Option<Cow<[crate::class::metadata::ClassMetadata]>> {
        <T as Object>::__object_children(*self)
    }

    fn __object_metadata(&self) -> Cow<crate::class::metadata::ClassMetadata> {
        <T as Object>::__object_metadata(*self)
    }

    fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf> {
        <T as Object>::__object_metadata_leaf(*self)
    }

    fn cursor(&self) -> <Self as Object>::Cursor {
        <T as Object>::cursor(*self)
    }
}
