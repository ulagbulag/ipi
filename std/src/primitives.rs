use std::borrow::Cow;

use crate::{class::Class, object::Object};

macro_rules! impl_class {
    (
        $impl_unsized:ty | $( $impls:ty ),* => $value_ty:ident
    ) => {
        const _: () = {
            impl_class!(@class $impl_unsized => $value_ty);
        };
        impl_class!(| $( $impls ),* => $value_ty);
    };
    (
        | $( $impls:ty ),* => $value_ty:ident
    ) => {
        $(
            impl_class!($impls => $value_ty);
        )*
    };
    (
        $impl:ty => $value_ty:ident
    ) => {
        const _: () = {
            impl_class!(@class $impl => $value_ty);
            impl_class!(@object $impl => $value_ty);
        };
    };
    (
        @class $impl:ty => $value_ty:ident
    ) => {
        impl Class for $impl {
            type Cursor = Cursor;

            fn __class_name() -> crate::class::metadata::ClassName {
                <<Self as Class>::Cursor as Class>::__class_name()
            }

            fn __class_doc() -> crate::class::metadata::ClassDoc {
                <<Self as Class>::Cursor as Class>::__class_doc()
            }

            fn __class_value_ty() -> ::ipi_core::value::ValueType {
                <<Self as Class>::Cursor as Class>::__class_value_ty()
            }

            fn __class_children() -> Option<Vec<crate::class::metadata::ClassMetadata>> {
                <<Self as Class>::Cursor as Class>::__class_children()
            }

            fn __class_metadata() -> crate::class::metadata::ClassMetadata {
                <<Self as Class>::Cursor as Class>::__class_metadata()
            }

            fn __class_metadata_leaf() -> crate::class::metadata::ClassLeaf {
                <<Self as Class>::Cursor as Class>::__class_metadata_leaf()
            }

            fn class_cursor() -> <Self as Class>::Cursor {
                <<Self as Class>::Cursor as Class>::class_cursor()
            }
        }

        #[derive(Clone, Default)]
        pub struct Cursor(crate::class::cursor::ClassCursorData);

        impl From<crate::class::cursor::ClassCursorData> for Cursor {
            fn from(value: crate::class::cursor::ClassCursorData) -> Self {
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

            fn __class_name() -> crate::class::metadata::ClassName {
                const NAME: &'static str = stringify!($impl);
                crate::class::metadata::ClassName::with_en_us(NAME)
            }

            fn __class_doc() -> crate::class::metadata::ClassDoc {
                None
            }

            fn __class_value_ty() -> ::ipi_core::value::ValueType {
                ::ipi_core::value::ValueType::$value_ty
            }

            fn __class_children() -> Option<Vec<crate::class::metadata::ClassMetadata>> {
                None
            }
        }

        impl Object for Cursor {
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

            fn __object_children(
                &self,
            ) -> Option<Cow<[crate::class::metadata::ClassMetadata]>> {
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
    (
        @object $impl:ty => $value_ty:ident
    ) => {
        impl Object for $impl {
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

            fn __object_children(
                &self,
            ) -> Option<Cow<[crate::class::metadata::ClassMetadata]>> {
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
    };
}

impl_class!(() => None);
impl_class!(bool => Bool);
impl_class!(i8 => I8);
impl_class!(i16 => I16);
impl_class!(i32 => I32);
impl_class!(i64 => I64);
impl_class!(u8 => U8);
impl_class!(u16 => U16);
impl_class!(u32 => U32);
impl_class!(u64 => U64);
impl_class!(f32 => F32);
impl_class!(f64 => U64);
impl_class!([u8] | Vec<u8> => Bytes);
impl_class!(str | String => String);
impl_class!(::ipi_core::value::text::Text => Text);
