use std::borrow::Cow;

use super::Object;
use crate::class::Class;

const _: () = {
    impl Object for () {
        type Cursor = <Self as Class>::Cursor;

        fn __object_name(&self) -> Cow<crate::class::metadata::ClassName> {
            Cow::Owned(<<Self as Class>::Cursor as Class>::__class_name())
        }

        fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc> {
            Cow::Owned(<<Self as Class>::Cursor as Class>::__class_doc())
        }

        fn __object_value_ty(&self) -> ::ipi_core::value::ValueType {
            <<Self as Class>::Cursor as Class>::__class_value_ty()
        }

        fn __object_children(&self) -> Option<Cow<[crate::class::metadata::ClassMetadata]>> {
            <<Self as Class>::Cursor as Class>::__class_children().map(Into::into)
        }

        fn __object_metadata(&self) -> Cow<crate::class::metadata::ClassMetadata> {
            Cow::Owned(<<Self as Class>::Cursor as Class>::__class_metadata())
        }

        fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf> {
            Cow::Owned(<<Self as Class>::Cursor as Class>::__class_metadata_leaf())
        }

        fn cursor(&self) -> <Self as Object>::Cursor {
            <<Self as Class>::Cursor as Class>::class_cursor()
        }
    }

    // impl Object for <() as Object>::Cursor {
    impl Object for crate::class::primitives::Cursor {
        type Cursor = Self;

        fn __object_name(&self) -> Cow<crate::class::metadata::ClassName> {
            Cow::Owned(<Self as Class>::__class_name())
        }

        fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc> {
            Cow::Owned(<Self as Class>::__class_doc())
        }

        fn __object_value_ty(&self) -> ::ipi_core::value::ValueType {
            <Self as Class>::__class_value_ty()
        }

        fn __object_children(&self) -> Option<Cow<[crate::class::metadata::ClassMetadata]>> {
            <Self as Class>::__class_children()
                .map(|e| e)
                .map(Into::into)
        }

        fn __object_metadata(&self) -> Cow<crate::class::metadata::ClassMetadata> {
            Cow::Owned(<Self as Class>::__class_metadata())
        }

        fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf> {
            Cow::Owned(<Self as Class>::__class_metadata_leaf())
        }

        fn cursor(&self) -> <Self as crate::class::Class>::Cursor {
            self.clone()
        }
    }
};
