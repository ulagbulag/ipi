pub mod cursor;
pub mod metadata;
pub mod primitives;

pub trait Class {
    type Cursor: ::core::fmt::Debug + Default;

    fn __class_name() -> self::metadata::ClassName;

    fn __class_doc() -> self::metadata::ClassDoc;

    fn __class_value_ty() -> ::ipi_core::value::ValueType;

    fn __class_children() -> Option<Vec<self::metadata::ClassMetadata>>;

    fn __class_metadata() -> self::metadata::ClassMetadata {
        self::metadata::ClassMetadata {
            leaf: <Self as Class>::__class_metadata_leaf(),
            children: <Self as Class>::__class_children(),
        }
    }

    fn __class_metadata_leaf() -> self::metadata::ClassLeaf {
        self::metadata::ClassLeaf {
            name: <Self as Class>::__class_name(),
            doc: <Self as Class>::__class_doc(),
            ty: <Self as Class>::__class_value_ty(),
        }
    }

    fn class_cursor() -> <Self as Class>::Cursor {
        <<Self as Class>::Cursor as Default>::default()
    }
}

impl<T> Class for &T
where
    T: Class,
{
    type Cursor = <T as Class>::Cursor;

    fn __class_name() -> self::metadata::ClassName {
        <T as Class>::__class_name()
    }

    fn __class_doc() -> self::metadata::ClassDoc {
        <T as Class>::__class_doc()
    }

    fn __class_value_ty() -> ::ipi_core::value::ValueType {
        <T as Class>::__class_value_ty()
    }

    fn __class_children() -> Option<Vec<self::metadata::ClassMetadata>> {
        <T as Class>::__class_children()
    }

    fn __class_metadata() -> self::metadata::ClassMetadata {
        <T as Class>::__class_metadata()
    }

    fn __class_metadata_leaf() -> self::metadata::ClassLeaf {
        <T as Class>::__class_metadata_leaf()
    }

    fn class_cursor() -> <Self as Class>::Cursor {
        <T as Class>::class_cursor()
    }
}
