use core::future::Future;

use std::io::ErrorKind;

use std::path::Path;
use std::path::PathBuf;

use tonic::Status;

/// Tries to get the content of the file specified by the [`Path`].
#[tonic::async_trait]
pub trait PathToRaw: Sync + Send {
    async fn path2raw<P>(&self, p: P) -> Result<Vec<u8>, std::io::Error>
    where
        P: AsRef<Path> + Send;

    async fn path_to_raw<P>(&self, p: P) -> Result<Vec<u8>, Status>
    where
        P: AsRef<Path> + Send,
    {
        let pat: &Path = p.as_ref();
        self.path2raw(pat).await.map_err(|e| match e.kind() {
            ErrorKind::NotFound => Status::not_found(format!("no such file({pat:#?}): {e}")),
            _ => Status::internal(format!("unable to read the file({pat:#?}): {e}")),
        })
    }
}

pub struct PathToRawFn<F> {
    path2raw: F,
}

#[tonic::async_trait]
impl<F, Fut> PathToRaw for PathToRawFn<F>
where
    F: Fn(PathBuf) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Vec<u8>, std::io::Error>> + Send,
{
    async fn path2raw<P>(&self, p: P) -> Result<Vec<u8>, std::io::Error>
    where
        P: AsRef<Path> + Send,
    {
        let pr: &Path = p.as_ref();
        let pb: PathBuf = pr.into();
        (self.path2raw)(pb).await
    }
}

pub fn path2raw_new_fn<F, Fut>(path2raw: F) -> impl PathToRaw
where
    F: Fn(PathBuf) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Vec<u8>, std::io::Error>> + Send,
{
    PathToRawFn { path2raw }
}
