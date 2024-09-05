//! Traits/functions to get the count from the readable.

use core::future::Future;
use core::marker::PhantomData;

use tokio::io::AsyncRead;

use tonic::Status;

/// Gets the count info from the readable.
#[tonic::async_trait]
pub trait ReadToCount: Sync + Send {
    type Read: AsyncRead + Sync + Send + Unpin;

    async fn read2count(&self, r: Self::Read) -> Result<u64, Status>;
}

pub struct ReadToCntFn<F, Read> {
    read2cnt: F,
    read: PhantomData<Read>,
}

#[tonic::async_trait]
impl<F, R, Fut> ReadToCount for ReadToCntFn<F, R>
where
    F: Fn(R) -> Fut + Send + Sync,
    R: AsyncRead + Sync + Send + Unpin,
    Fut: Future<Output = Result<u64, Status>> + Send,
{
    type Read = R;
    async fn read2count(&self, r: Self::Read) -> Result<u64, Status> {
        (self.read2cnt)(r).await
    }
}

/// Creates a [`ReadToCount`] from the function `read2cnt`.
pub fn read2cnt_fn_new<F, R, Fut>(read2cnt: F) -> impl ReadToCount<Read = R>
where
    F: Fn(R) -> Fut + Send + Sync,
    R: AsyncRead + Sync + Send + Unpin,
    Fut: Future<Output = Result<u64, Status>> + Send,
{
    ReadToCntFn {
        read2cnt,
        read: PhantomData,
    }
}
