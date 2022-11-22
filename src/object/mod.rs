pub mod data;

use std::borrow::Cow;

pub trait Object {
    type Cursor: Clone + ::core::fmt::Debug + Default;

    fn __object_name(&self) -> Cow<crate::class::metadata::ClassName>;

    fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc>;

    fn __object_value_ty(&self) -> ::ipi::value::ValueType;

    fn __object_metadata(&self) -> crate::class::metadata::ClassMetadata;

    fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf>;

    fn cursor(&self) -> <Self as Object>::Cursor {
        <<Self as Object>::Cursor as Default>::default()
    }
}

impl<T> Object for &T
where
    T: ?Sized + Object,
{
    type Cursor = <T as Object>::Cursor;

    fn __object_name(&self) -> Cow<crate::class::metadata::ClassName> {
        <T as Object>::__object_name(*self)
    }

    fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc> {
        <T as Object>::__object_doc(*self)
    }

    fn __object_value_ty(&self) -> ::ipi::value::ValueType {
        <T as Object>::__object_value_ty(*self)
    }

    fn __object_metadata(&self) -> crate::class::metadata::ClassMetadata {
        <T as Object>::__object_metadata(*self)
    }

    fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf> {
        <T as Object>::__object_metadata_leaf(*self)
    }

    fn cursor(&self) -> <Self as Object>::Cursor {
        <T as Object>::cursor(*self)
    }
}

pub trait ToObjectData
where
    Self: Object,
{
    fn __to_object_attention(&self) -> crate::attention::AttentionUnit {
        crate::attention::AttentionUnit::Usually
    }

    fn __to_object_value(&self) -> Option<::ipi::value::Value>;

    fn __to_object_children(&self) -> Option<Vec<self::data::ObjectData>>;

    fn __to_object_data(&self) -> self::data::ObjectData {
        self::data::ObjectData {
            leaf: self.__object_metadata_leaf().into_owned(),
            attention: self.__to_object_attention(),
            confidence: self.__to_object_attention(),
            value: self.__to_object_value(),
            children: self.__to_object_children(),
        }
    }

    fn __get_object_value(&self, path: &[::ipi::value::text::Text]) -> Option<::ipi::value::Value>;

    fn __get_object_data(
        &self,
        path: &[::ipi::value::text::Text],
    ) -> Option<self::data::ObjectData>;
}

impl<T> ToObjectData for &T
where
    T: ?Sized + ToObjectData,
{
    fn __to_object_attention(&self) -> crate::attention::AttentionUnit {
        <T as ToObjectData>::__to_object_attention(*self)
    }

    fn __to_object_value(&self) -> Option<::ipi::value::Value> {
        <T as ToObjectData>::__to_object_value(*self)
    }

    fn __to_object_children(&self) -> Option<Vec<self::data::ObjectData>> {
        <T as ToObjectData>::__to_object_children(*self)
    }

    fn __to_object_data(&self) -> self::data::ObjectData {
        <T as ToObjectData>::__to_object_data(*self)
    }

    fn __get_object_value(&self, path: &[::ipi::value::text::Text]) -> Option<::ipi::value::Value> {
        <T as ToObjectData>::__get_object_value(*self, path)
    }

    fn __get_object_data(
        &self,
        path: &[::ipi::value::text::Text],
    ) -> Option<self::data::ObjectData> {
        <T as ToObjectData>::__get_object_data(*self, path)
    }
}

pub trait IntoObjectData
where
    Self: ToObjectData + Sized,
{
    fn __into_object_attention(self) -> crate::attention::AttentionUnit {
        self.__to_object_attention()
    }

    fn __into_object_value(self) -> Option<::ipi::value::Value> {
        self.__to_object_value()
    }

    fn __into_object_children(self) -> Option<Vec<self::data::ObjectData>> {
        self.__to_object_children()
    }

    fn __into_object_data(self) -> self::data::ObjectData {
        self.__to_object_data()
    }
}
