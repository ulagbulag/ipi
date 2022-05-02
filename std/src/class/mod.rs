pub mod cursor;
pub mod metadata;
pub mod primitives;

use ipi_core::anyhow::Result;

pub trait Class {
    type Cursor: ::core::fmt::Debug;

    fn __class_name() -> Result<self::metadata::ClassName>;

    fn __class_doc() -> Result<self::metadata::ClassDoc>;

    fn __class_value_ty() -> ::ipi_core::value::ValueType;

    fn __class_children() -> Result<Vec<self::metadata::ClassMetadata>>;

    fn __class_metadata() -> Result<self::metadata::ClassMetadata>;

    fn __class_metadata_leaf() -> Result<self::metadata::ClassLeaf>;

    fn cursor() -> <Self as Class>::Cursor;
}
