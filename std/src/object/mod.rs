use std::borrow::Cow;

use ipi_core::anyhow::Result;

pub trait Object {
    type Cursor: Clone + ::core::fmt::Debug;

    fn __object_name(&self) -> Result<Cow<crate::class::metadata::ClassName>>;

    fn __object_doc(&self) -> Result<Cow<crate::class::metadata::ClassDoc>>;

    fn __object_value_ty(&self) -> ::ipi_core::value::ValueType;

    fn __object_children(&self) -> Result<Cow<[crate::class::metadata::ClassMetadata]>>;

    fn __object_metadata(&self) -> Result<Cow<crate::class::metadata::ClassMetadata>>;

    fn __object_metadata_leaf(&self) -> Result<Cow<crate::class::metadata::ClassLeaf>>;

    fn cursor(&self) -> Cow<<Self as Object>::Cursor>;
}
