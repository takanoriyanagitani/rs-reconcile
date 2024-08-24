use core::future::Future;
use core::marker::PhantomData;

use std::io::ErrorKind;

use std::path::Path;
use std::path::PathBuf;

use tokio::io::AsyncRead;

use tonic::Status;

/// Tries to get a readable(e.g, a file) from the [`Path`].
#[tonic::async_trait]
pub trait PathToRead: Sync + Send {
    type Read: AsyncRead + Sync + Send + Unpin;

    async fn path2read<P>(&self, p: P) -> Result<Self::Read, std::io::Error>
    where
        P: AsRef<Path> + Send;

    async fn path_to_read<P>(&self, p: P) -> Result<Self::Read, Status>
    where
        P: AsRef<Path> + Send,
    {
        let pat: &Path = p.as_ref();
        self.path2read(pat).await.map_err(|e| match e.kind() {
            ErrorKind::NotFound => Status::not_found(format!("file does not exist({pat:#?}): {e}")),
            _ => Status::internal(format!("unable to read({pat:#?}): {e}")),
        })
    }
}

pub struct PathToReadFn<F, R> {
    path2read: F,
    read: PhantomData<R>,
}

#[tonic::async_trait]
impl<F, R, Fut> PathToRead for PathToReadFn<F, R>
where
    F: Fn(PathBuf) -> Fut + Sync + Send,
    R: AsyncRead + Sync + Send + Unpin,
    Fut: Future<Output = Result<R, std::io::Error>> + Send,
{
    type Read = R;

    async fn path2read<P>(&self, p: P) -> Result<Self::Read, std::io::Error>
    where
        P: AsRef<Path> + Send,
    {
        let pat: &Path = p.as_ref();
        (self.path2read)(pat.into()).await
    }
}

/// Creates a [`PathToRead`] from the function `path2read`.
pub fn path2read_fn_new<F, R, Fut>(path2read: F) -> impl PathToRead<Read = R>
where
    F: Fn(PathBuf) -> Fut + Sync + Send,
    R: AsyncRead + Sync + Send + Unpin,
    Fut: Future<Output = Result<R, std::io::Error>> + Send,
{
    PathToReadFn {
        path2read,
        read: PhantomData,
    }
}
