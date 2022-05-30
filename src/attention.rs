#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub enum AttentionUnit {
    Always,
    Virtually,
    Usually,
    Sometimes,
    Ever,
    Never,
}

impl ::ipi::signed::IsSigned for AttentionUnit {}
