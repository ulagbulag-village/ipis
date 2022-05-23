use bytecheck::CheckBytes;
use ipi::{
    signed::IsSigned,
    value::{
        hash::Hash,
        text::{Text, TextHash},
    },
};
use rkyv::{Archive, Deserialize, Serialize};

use crate::path::Path;

#[derive(Clone, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct WordKey {
    pub namespace: String,
    pub kind: String,
    pub text: Text,
}

impl IsSigned for WordKey {}

impl ::core::fmt::Debug for WordKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.text, f)
    }
}

impl ::core::fmt::Display for WordKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.text, f)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct Word {
    pub key: WordKey,
    pub relpath: bool,
    pub path: Path,
}

impl IsSigned for Word {}

impl ::core::fmt::Debug for Word {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.key, f)
    }
}

impl ::core::fmt::Display for Word {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.key, f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct WordKeyHash {
    pub namespace: Hash,
    pub kind: Hash,
    pub text: TextHash,
}

impl IsSigned for WordKeyHash {}

impl From<WordKey> for WordKeyHash {
    fn from(value: WordKey) -> Self {
        Self {
            namespace: Hash::with_str(&value.namespace),
            kind: Hash::with_str(&value.kind),
            text: value.text.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct WordHash {
    pub key: WordKeyHash,
    pub relpath: bool,
    pub path: Path,
}

impl IsSigned for WordHash {}

impl From<Word> for WordHash {
    fn from(value: Word) -> Self {
        Self {
            key: value.key.into(),
            relpath: value.relpath,
            path: value.path,
        }
    }
}
