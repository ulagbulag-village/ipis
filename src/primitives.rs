use std::borrow::Cow;

use crate::{
    class::Class,
    object::{IntoObjectData, Object, ToObjectData},
};

macro_rules! impl_class {
    (
        $impl_unsized:ty | $( $impls:ty ),* => $value_move:ident ( $value_ty:ident )
    ) => {
        const _: () = {
            impl_class!(@class $impl_unsized => $value_ty);
        };
        impl_class!(| $( $impls ),* => $value_move ($value_ty));
    };
    (
        | $( $impls:ty ),* => $value_move:ident ( $value_ty:ident )
    ) => {
        $(
            impl_class!($impls => $value_move ($value_ty));
        )*
    };
    (
        | $( $impls:ty ),* => $value_ty:ident
    ) => {
        $(
            impl_class!($impls => $value_ty);
        )*
    };
    (
        $impl:ty => $value_move:ident ( $value_ty:ident )
    ) => {
        impl_class!($impl => $value_ty);
        impl_class!(@to_object_value $impl => $value_move ($value_ty));
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

            fn __class_value_ty() -> ::ipi::value::ValueType {
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

            fn __class_value_ty() -> ::ipi::value::ValueType {
                ::ipi::value::ValueType::$value_ty
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

            fn __object_value_ty(&self) -> ::ipi::value::ValueType {
                <Self as Class>::__class_value_ty()
            }

            fn __object_metadata(&self) -> crate::class::metadata::ClassMetadata {
                <Self as Class>::__class_metadata()
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

            fn __object_value_ty(&self) -> ::ipi::value::ValueType {
                <<Self as Class>::Cursor as Class>::__class_value_ty()
            }

            fn __object_metadata(&self) -> crate::class::metadata::ClassMetadata {
                <<Self as Class>::Cursor as Class>::__class_metadata()
            }

            fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf> {
                Cow::Owned(<<Self as Class>::Cursor as Class>::__class_metadata_leaf())
            }

            fn cursor(&self) -> <Self as Object>::Cursor {
                <<Self as Class>::Cursor as Class>::class_cursor()
            }
        }
    };
    (
        @to_object_value $impl:ty => $value_move:ident ( $value_ty:ident )
    ) => {
        const _: () = {
            impl ToObjectData for $impl {
                fn __to_object_value(&self) -> Option<::ipi::value::Value> {
                    Some(impl_class!(@to_object_value $impl => $value_move ($value_ty) => self))
                }

                fn __to_object_children(&self) -> Option<Vec<crate::object::data::ObjectData>> {
                    None
                }

                fn __get_object_value(&self, path: &[::ipi::value::text::Text]) -> Option<::ipi::value::Value> {
                    if path.is_empty() {
                        self.__to_object_value()
                    } else {
                        None
                    }
                }

                fn __get_object_data(&self, path: &[::ipi::value::text::Text]) -> Option<crate::object::data::ObjectData> {
                    if path.is_empty() {
                        Some(self.__to_object_data())
                    } else {
                        None
                    }
                }
            }

            impl IntoObjectData for $impl {
                fn __into_object_value(self) -> Option<::ipi::value::Value> {
                    Some(impl_class!(@into_object_value $impl => $value_move ($value_ty) => self))
                }

                fn __into_object_children(self) -> Option<Vec<crate::object::data::ObjectData>> {
                    None
                }

                fn __into_object_data(self) -> crate::object::data::ObjectData {
                    crate::object::data::ObjectData {
                        leaf: <$impl as Class>::__class_metadata_leaf(),
                        value: self.__into_object_value(),
                        children: None,
                    }
                }
            }
        };
    };
    (
        @to_object_value $impl:ty => None ( $value_ty:ident ) => $value:expr
    ) => {
        ::ipi::value::Value::$value_ty
    };
    (
        @to_object_value $impl:ty => Copy ( $value_ty:ident ) => $value:expr
    ) => {
        ::ipi::value::Value::$value_ty(*$value)
    };
    (
        @to_object_value $impl:ty => Clone ( $value_ty:ident ) => $value:expr
    ) => {
        ::ipi::value::Value::$value_ty($value.clone())
    };
    (
        @into_object_value $impl:ty => None ( $value_ty:ident ) => $value:expr
    ) => {
        ::ipi::value::Value::$value_ty
    };
    (
        @into_object_value $impl:ty => Copy ( $value_ty:ident ) => $value:expr
    ) => {
        ::ipi::value::Value::$value_ty($value)
    };
    (
        @into_object_value $impl:ty => Clone ( $value_ty:ident ) => $value:expr
    ) => {
        ::ipi::value::Value::$value_ty($value)
    };
}

impl_class!(() => None(None));
impl_class!(bool => Copy(Bool));
impl_class!(i8 => Copy(I8));
impl_class!(i16 => Copy(I16));
impl_class!(i32 => Copy(I32));
impl_class!(i64 => Copy(I64));
impl_class!(u8 => Copy(U8));
impl_class!(u16 => Copy(U16));
impl_class!(u32 => Copy(U32));
impl_class!(u64 => Copy(U64));
impl_class!(f32 => Copy(F32));
impl_class!(f64 => Copy(F64));
impl_class!([u8] | Vec<u8> => Clone(Bytes));
impl_class!(String => Clone(String));
impl_class!(::ipi::value::text::Text => Clone(Text));
