use bytecheck::CheckBytes;
use ipi::value::Value;

use crate::{attention::AttentionUnit, class::metadata::ClassLeaf};

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(bound(serialize = "
    __S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, PartialEq))]
pub struct ObjectData {
    pub leaf: ClassLeaf,
    pub attention: AttentionUnit,
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
        CheckBytes::<__C>::check_bytes(::core::ptr::addr_of!((*value).attention), context)
            .map_err(|e| ::bytecheck::StructCheckError {
                field_name: stringify!(attention),
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
