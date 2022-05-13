use ipi::value::hash::Hash;

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]

pub struct Path {
    pub value: Hash,
    pub len: u64,
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(bound(archive = "
    <Path as ::rkyv::Archive>::Archived: ::core::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + ::core::hash::Hash,
"))]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]

pub struct DynPath<Path = Option<self::Path>> {
    pub kind: Hash,
    pub word: Hash,
    pub path: Path,
}

impl From<DynPath<Path>> for DynPath {
    fn from(value: DynPath<Path>) -> Self {
        Self {
            kind: value.kind,
            word: value.word,
            path: Some(value.path),
        }
    }
}
