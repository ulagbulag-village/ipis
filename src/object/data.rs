use std::borrow::Cow;

use bytecheck::CheckBytes;
use ipi::value::{Value, ValueType};

use crate::{attention::AttentionUnit, class::metadata::ClassLeaf};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Archive,
    Serialize,
    Deserialize,
    ::serde::Serialize,
    ::serde::Deserialize,
)]
#[archive(bound(serialize = "
    __S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, PartialEq))]
pub struct ObjectData {
    pub leaf: ClassLeaf,
    pub attention: AttentionUnit,
    pub confidence: AttentionUnit,
    pub value: Option<Value>,
    #[omit_bounds]
    pub children: Option<Vec<ObjectData>>,
}

impl ::ipi::signed::IsSigned for ObjectData {}

impl<__C> CheckBytes<__C> for ArchivedObjectData
where
    __C: ::rkyv::validation::ArchiveContext,
    <__C as ::rkyv::Fallible>::Error: ::std::error::Error,
{
    type Error = ::bytecheck::StructCheckError;

    unsafe fn check_bytes<'__bytecheck>(
        value: *const Self,
        context: &mut __C,
    ) -> Result<&'__bytecheck Self, Self::Error> {
        CheckBytes::<__C>::check_bytes(::core::ptr::addr_of!((*value).leaf), context).map_err(
            |e| ::bytecheck::StructCheckError {
                field_name: stringify!(leaf),
                inner: ::bytecheck::ErrorBox::new(e),
            },
        )?;
        CheckBytes::<__C>::check_bytes(::core::ptr::addr_of!((*value).confidence), context)
            .map_err(|e| ::bytecheck::StructCheckError {
                field_name: stringify!(confidence),
                inner: ::bytecheck::ErrorBox::new(e),
            })?;
        CheckBytes::<__C>::check_bytes(::core::ptr::addr_of!((*value).value), context).map_err(
            |e| ::bytecheck::StructCheckError {
                field_name: stringify!(value),
                inner: ::bytecheck::ErrorBox::new(e),
            },
        )?;
        CheckBytes::<__C>::check_bytes(::core::ptr::addr_of!((*value).children), context).map_err(
            |e| ::bytecheck::StructCheckError {
                field_name: stringify!(children),
                inner: ::bytecheck::ErrorBox::new(e),
            },
        )?;
        Ok(&*value)
    }
}

impl super::Object for ObjectData {
    type Cursor = crate::class::cursor::ClassCursorData;

    fn __object_name(&self) -> Cow<crate::class::metadata::ClassName> {
        Cow::Borrowed(&self.leaf.name)
    }

    fn __object_doc(&self) -> Cow<crate::class::metadata::ClassDoc> {
        Cow::Borrowed(&self.leaf.doc)
    }

    fn __object_value_ty(&self) -> ValueType {
        self.leaf.ty
    }

    fn __object_metadata(&self) -> crate::class::metadata::ClassMetadata {
        crate::class::metadata::ClassMetadata {
            leaf: self.leaf.clone(),
            children: self.children.as_ref().map(|children| {
                children
                    .iter()
                    .map(super::Object::__object_metadata)
                    .collect()
            }),
        }
    }

    fn __object_metadata_leaf(&self) -> Cow<crate::class::metadata::ClassLeaf> {
        Cow::Borrowed(&self.leaf)
    }
}

impl super::ToObjectData for ObjectData {
    fn __to_object_attention(&self) -> crate::attention::AttentionUnit {
        self.attention
    }

    fn __to_object_value(&self) -> Option<Value> {
        self.value.clone()
    }

    fn __to_object_children(&self) -> Option<Vec<ObjectData>> {
        self.children.clone()
    }

    fn __to_object_data(&self) -> ObjectData {
        self.clone()
    }

    fn __get_object_value(&self, path: &[::ipi::value::text::Text]) -> Option<Value> {
        if path.is_empty() {
            self.__to_object_value()
        } else {
            self.children
                .as_ref()
                .and_then(|children| children.iter().find(|child| child.leaf.name == path[0]))
                .and_then(|child| child.__get_object_value(&path[1..]))
        }
    }

    fn __get_object_data(&self, path: &[::ipi::value::text::Text]) -> Option<ObjectData> {
        if path.is_empty() {
            Some(self.__to_object_data())
        } else {
            self.children
                .as_ref()
                .and_then(|children| children.iter().find(|child| child.leaf.name == path[0]))
                .and_then(|child| child.__get_object_data(&path[1..]))
        }
    }
}
