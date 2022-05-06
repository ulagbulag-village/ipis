use core::{marker::PhantomPinned, pin::Pin, ptr::NonNull};

use bytecheck::CheckBytes;
use ipi::anyhow::{anyhow, Result};
use rkyv::{
    de::deserializers::SharedDeserializeMap, validation::validators::DefaultValidator, Archive,
    Deserialize,
};

pub type Pinned<T> = Pin<Box<PinnedInner<T>>>;

pub struct PinnedInner<T>
where
    T: Archive,
{
    data: Vec<u8>,
    archived: NonNull<<T as Archive>::Archived>,
    _pin: PhantomPinned,
}

unsafe impl<T> Send for PinnedInner<T> where T: Archive {}

impl<T> ::core::ops::Deref for PinnedInner<T>
where
    T: Archive,
{
    type Target = <T as Archive>::Archived;

    fn deref(&self) -> &Self::Target {
        // we know this is safe because it's already initialized
        unsafe { self.archived.as_ref() }
    }
}

impl<T> PinnedInner<T>
where
    T: Archive,
{
    pub fn new(data: Vec<u8>) -> Result<Pinned<T>>
    where
        <T as Archive>::Archived: for<'a> CheckBytes<DefaultValidator<'a>>,
    {
        let mut boxed = Box::pin(Self {
            data,
            archived: NonNull::dangling(),
            _pin: Default::default(),
        });

        let archived = NonNull::from(
            ::rkyv::check_archived_root::<T>(&boxed.data)
                .map_err(|_| anyhow!("failed to check the archived bytes"))?,
        );
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).archived = archived;
        }
        Ok(boxed)
    }

    pub fn deserialize_owned(data: impl AsRef<[u8]>) -> Result<T>
    where
        <T as Archive>::Archived:
            for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
    {
        let archived = ::rkyv::check_archived_root::<T>(data.as_ref())
            .map_err(|_| anyhow!("failed to check the archived bytes"))?;

        Deserialize::<T, _>::deserialize(archived, &mut SharedDeserializeMap::default())
            .map_err(Into::into)
    }

    pub fn deserialize_into(&self) -> Result<T>
    where
        <T as Archive>::Archived: Deserialize<T, SharedDeserializeMap>,
    {
        Deserialize::<T, _>::deserialize(&**self, &mut SharedDeserializeMap::default())
            .map_err(Into::into)
    }
}
