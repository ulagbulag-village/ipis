use core::pin::Pin;
use std::sync::Arc;

use bytecheck::CheckBytes;
use ipi::{
    anyhow::Result,
    signed::{IsSigned, Serializer},
};
use rkyv::{
    de::deserializers::SharedDeserializeMap, validation::validators::DefaultValidator, AlignedVec,
    Archive, Deserialize, Serialize,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::Mutex,
};

use crate::pin::PinnedInner;

pub type DynStreamMut<'io, T> = Mutex<DynStream<'io, T>>;

pub enum DynStream<'io, T>
where
    T: Archive,
{
    Archived(PinnedInner<T>),
    Borrowed(&'io T),
    BorrowedSlice(&'io [u8]),
    Owned(T),
    OwnedAlignedVec(AlignedVec),
    OwnedVec(Vec<u8>),
    ArcVec(Arc<Vec<u8>>),
    Stream {
        len: u64,
        recv: Pin<Box<dyn AsyncRead + Send + 'static>>,
    },
}

impl<T> DynStream<'static, T>
where
    T: Archive + 'static,
{
    pub async fn recv<R>(mut src: R) -> Result<Self>
    where
        T: IsSigned,
        R: AsyncRead + Unpin,
    {
        let len = src.read_u64().await?;
        Self::recv_exact(src, len).await.map(Self::OwnedVec)
    }
}

impl<'io, T> DynStream<'io, T>
where
    T: Archive,
{
    async fn recv_exact<R>(mut src: R, len: u64) -> Result<Vec<u8>>
    where
        T: IsSigned,
        R: AsyncRead + Unpin,
    {
        let mut buf = vec![0; len.try_into()?];
        src.read_exact(&mut buf).await?;
        Ok(buf)
    }
}

impl<'io, T> DynStream<'io, T>
where
    T: Archive + Serialize<Serializer> + IsSigned + Clone,
    <T as Archive>::Archived:
        for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    pub async fn as_ref(&mut self) -> Result<&T> {
        match self {
            Self::Archived(data) => {
                let data = data.deserialize_into()?;
                *self = Self::Owned(data);
                match self {
                    Self::Owned(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            Self::Borrowed(data) => Ok(data),
            Self::BorrowedSlice(data) => {
                let data = PinnedInner::<T>::deserialize_owned(data)?;
                *self = Self::Owned(data);
                match self {
                    Self::Owned(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            Self::Owned(data) => Ok(data),
            Self::OwnedAlignedVec(data) => {
                let data = PinnedInner::<T>::deserialize_owned(data)?;
                *self = Self::Owned(data);
                match self {
                    Self::Owned(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            Self::OwnedVec(data) => {
                let data = PinnedInner::<T>::deserialize_owned(data)?;
                *self = Self::Owned(data);
                match self {
                    Self::Owned(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            Self::ArcVec(data) => {
                let data = PinnedInner::<T>::deserialize_owned(&**data)?;
                *self = Self::Owned(data);
                match self {
                    Self::Owned(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            Self::Stream { len, recv } => {
                // recv data
                let buf = Self::recv_exact(recv, *len).await?;

                let data = PinnedInner::<T>::deserialize_owned(buf)?;
                *self = Self::Owned(data);
                match self {
                    Self::Owned(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub async fn to_owned(&mut self) -> Result<T> {
        match self {
            Self::Archived(data) => {
                let data = PinnedInner::<T>::deserialize_from_archived(data)?;
                *self = Self::Owned(data.clone());
                Ok(data)
            }
            Self::Borrowed(data) => Ok(data.clone()),
            Self::BorrowedSlice(data) => PinnedInner::<T>::deserialize_owned(data),
            Self::Owned(data) => Ok(data.clone()),
            Self::OwnedAlignedVec(data) => PinnedInner::<T>::deserialize_owned(data),
            Self::OwnedVec(data) => PinnedInner::<T>::deserialize_owned(data),
            Self::ArcVec(data) => PinnedInner::<T>::deserialize_owned(&**data),
            Self::Stream { len, recv } => {
                // recv data
                let buf = Self::recv_exact(recv, *len).await?;

                let data = PinnedInner::<T>::deserialize_owned(buf)?;
                *self = Self::Owned(data.clone());
                Ok(data)
            }
        }
    }

    pub async fn into_owned(self) -> Result<T> {
        match self {
            Self::Archived(data) => PinnedInner::<T>::deserialize_from_archived(&data),
            Self::Borrowed(data) => Ok(data.clone()),
            Self::BorrowedSlice(data) => PinnedInner::<T>::deserialize_owned(data),
            Self::Owned(data) => Ok(data),
            Self::OwnedAlignedVec(data) => PinnedInner::<T>::deserialize_owned(data),
            Self::OwnedVec(data) => PinnedInner::<T>::deserialize_owned(data),
            Self::ArcVec(data) => PinnedInner::<T>::deserialize_owned(&**data),
            Self::Stream { len, recv } => {
                // recv data
                let buf = Self::recv_exact(recv, len).await?;

                PinnedInner::<T>::deserialize_owned(buf)
            }
        }
    }

    pub async fn copy_to<W>(&mut self, mut dst: W) -> Result<()>
    where
        T: IsSigned,
        W: AsyncWrite + Unpin,
    {
        match self {
            Self::Archived(data) => {
                let len = data.as_ref().len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(data.as_ref()).await.map_err(Into::into)
            }
            Self::Borrowed(data) => {
                let data = ::rkyv::to_bytes(*data)?;
                let len = data.len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(&data).await.map_err(Into::into)
            }
            Self::BorrowedSlice(data) => {
                let len = data.len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(data).await.map_err(Into::into)
            }
            Self::Owned(data) => {
                let data = ::rkyv::to_bytes(data)?;
                let len = data.len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(&data).await.map_err(Into::into)
            }
            Self::OwnedAlignedVec(data) => {
                let len = data.len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(data).await.map_err(Into::into)
            }
            Self::OwnedVec(data) => {
                let len = data.len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(data).await.map_err(Into::into)
            }
            Self::ArcVec(data) => {
                let len = data.len().try_into()?;

                dst.write_u64(len).await?;
                dst.write_all(data).await.map_err(Into::into)
            }
            Self::Stream { len, recv } => {
                dst.write_u64(*len).await?;
                ::tokio::io::copy(recv, &mut dst)
                    .await
                    .map(|_| ())
                    .map_err(Into::into)
            }
        }
    }

    pub async fn serialize_inner(&mut self) -> Result<()> {
        match self {
            Self::Archived(_) => Ok(()),
            Self::Borrowed(data) => {
                let data = ::rkyv::to_bytes(*data)?;
                *self = Self::OwnedAlignedVec(data);
                Ok(())
            }
            Self::BorrowedSlice(_) => Ok(()),
            Self::Owned(data) => {
                let data = ::rkyv::to_bytes(data)?;
                *self = Self::OwnedAlignedVec(data);
                Ok(())
            }
            Self::OwnedAlignedVec(_) => Ok(()),
            Self::OwnedVec(_) => Ok(()),
            Self::ArcVec(_) => Ok(()),
            Self::Stream { .. } => Ok(()),
        }
    }
}
