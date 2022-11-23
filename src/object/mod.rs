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

pub trait ToObjectData<Metadata>
where
    Self: Object,
    Metadata: Default,
{
    fn __to_object_metadata(&self) -> Metadata {
        Default::default()
    }

    fn __to_object_value(&self) -> Option<::ipi::value::Value>;

    fn __to_object_children(&self) -> Option<Vec<self::data::ObjectData<Metadata>>>;

    fn __to_object_data(&self) -> self::data::ObjectData<Metadata> {
        self::data::ObjectData {
            metadata: self.__to_object_metadata(),
            leaf: self.__object_metadata_leaf().into_owned(),
            value: self.__to_object_value(),
            children: self.__to_object_children(),
        }
    }

    fn __get_object_value(&self, path: &[::ipi::value::text::Text]) -> Option<::ipi::value::Value>;

    fn __get_object_data(
        &self,
        path: &[::ipi::value::text::Text],
    ) -> Option<self::data::ObjectData<Metadata>>;
}

impl<T, Metadata> ToObjectData<Metadata> for &T
where
    T: ?Sized + ToObjectData<Metadata>,
    Metadata: Default,
{
    fn __to_object_metadata(&self) -> Metadata {
        <T as ToObjectData<Metadata>>::__to_object_metadata(*self)
    }

    fn __to_object_value(&self) -> Option<::ipi::value::Value> {
        <T as ToObjectData<Metadata>>::__to_object_value(*self)
    }

    fn __to_object_children(&self) -> Option<Vec<self::data::ObjectData<Metadata>>> {
        <T as ToObjectData<Metadata>>::__to_object_children(*self)
    }

    fn __to_object_data(&self) -> self::data::ObjectData<Metadata> {
        <T as ToObjectData<Metadata>>::__to_object_data(*self)
    }

    fn __get_object_value(&self, path: &[::ipi::value::text::Text]) -> Option<::ipi::value::Value> {
        <T as ToObjectData<Metadata>>::__get_object_value(*self, path)
    }

    fn __get_object_data(
        &self,
        path: &[::ipi::value::text::Text],
    ) -> Option<self::data::ObjectData<Metadata>> {
        <T as ToObjectData<Metadata>>::__get_object_data(*self, path)
    }
}

pub trait IntoObjectData<Metadata>
where
    Self: ToObjectData<Metadata> + Sized,
    Metadata: Default,
{
    fn __into_object_value(self) -> Option<::ipi::value::Value> {
        self.__to_object_value()
    }

    fn __into_object_children(self) -> Option<Vec<self::data::ObjectData<Metadata>>> {
        self.__to_object_children()
    }

    fn __into_object_data(self) -> self::data::ObjectData<Metadata> {
        self.__to_object_data()
    }
}
