#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, Hash))]
pub enum AccountType {
    Any,
    IPDIS,
    IPIIS,
    IPNIS,
    IPQIS,
    IPSIS,
    IPWIS,
}
