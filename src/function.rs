use ipi::value::text::Text;

use crate::{class::metadata::ClassMetadata, path::DynPath};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(bound(archive = "
    <IO as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq,
    <Lambda as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq,
"))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct Function<IO = ClassMetadata, Lambda = DynPath> {
    pub inputs: IO,
    pub outputs: IO,
    pub lambda: Lambda,
}

impl<IO, Lambda> ::ipi::signed::IsSigned for Function<IO, Lambda> {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub enum DynFunction {
    Text(Text),
    Raw(Box<Function>),
}

impl ::ipi::signed::IsSigned for DynFunction {}
