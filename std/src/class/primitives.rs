use super::Class;

#[derive(Clone, Default)]
pub struct Cursor(super::cursor::ClassCursorData);

const _: () = {
    impl Class for () {
        type Cursor = Cursor;

        fn __class_name() -> super::metadata::ClassName {
            <<Self as Class>::Cursor as Class>::__class_name()
        }

        fn __class_doc() -> super::metadata::ClassDoc {
            <<Self as Class>::Cursor as Class>::__class_doc()
        }

        fn __class_value_ty() -> ::ipi_core::value::ValueType {
            <<Self as Class>::Cursor as Class>::__class_value_ty()
        }

        fn __class_children() -> Option<Vec<super::metadata::ClassMetadata>> {
            <<Self as Class>::Cursor as Class>::__class_children()
        }

        fn __class_metadata() -> super::metadata::ClassMetadata {
            <<Self as Class>::Cursor as Class>::__class_metadata()
        }

        fn __class_metadata_leaf() -> super::metadata::ClassLeaf {
            <<Self as Class>::Cursor as Class>::__class_metadata_leaf()
        }

        fn class_cursor() -> <Self as Class>::Cursor {
            <<Self as Class>::Cursor as Class>::class_cursor()
        }
    }

    // #[derive(Clone, Default)]
    // pub struct Cursor(super::cursor::ClassCursorData);

    impl From<super::cursor::ClassCursorData> for Cursor {
        fn from(value: super::cursor::ClassCursorData) -> Self {
            Self(value)
        }
    }

    impl ::core::fmt::Debug for Cursor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            ::core::fmt::Debug::fmt(&self.0, f)
        }
    }

    impl ::core::fmt::Display for Cursor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            ::core::fmt::Display::fmt(&self.0, f)
        }
    }

    impl Class for Cursor {
        type Cursor = Self;

        fn __class_name() -> super::metadata::ClassName {
            super::metadata::ClassName::with_en_us("()")
        }

        fn __class_doc() -> super::metadata::ClassDoc {
            None
        }

        fn __class_value_ty() -> ::ipi_core::value::ValueType {
            ::ipi_core::value::ValueType::Bool
        }

        fn __class_children() -> Option<Vec<super::metadata::ClassMetadata>> {
            None
        }
    }
};
