use bytecheck::CheckBytes;
use ipi::value::{text::Text, ValueType};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(serialize = "
    __S: ::rkyv::ser::ScratchSpace + ::rkyv::ser::Serializer,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, PartialEq))]
pub struct ClassMetadata {
    pub leaf: ClassLeaf,
    #[omit_bounds]
    pub children: Option<Vec<ClassMetadata>>,
}

impl ::ipi::signed::IsSigned for ClassMetadata {}

impl ::core::ops::Deref for ClassMetadata {
    type Target = ClassLeaf;

    fn deref(&self) -> &Self::Target {
        &self.leaf
    }
}

impl<__C> CheckBytes<__C> for ArchivedClassMetadata
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
        CheckBytes::<__C>::check_bytes(::core::ptr::addr_of!((*value).children), context).map_err(
            |e| ::bytecheck::StructCheckError {
                field_name: stringify!(children),
                inner: ::bytecheck::ErrorBox::new(e),
            },
        )?;
        Ok(&*value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct ClassLeaf {
    pub name: ClassName,
    pub doc: ClassDoc,
    pub ty: ValueType,
}

impl ::ipi::signed::IsSigned for ClassLeaf {}

pub type ClassName = Text;
pub type ClassDoc = Option<Text>;
