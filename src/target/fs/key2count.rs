use std::path::PathBuf;

use tonic::Status;

use crate::count::CountSource;

use super::key2path::KeyToPath;
use super::path2read::PathToRead;
use super::read2count::ReadToCount;

pub struct KeyToCnt<K, P, R> {
    pub key2path: K,
    pub path2read: P,
    pub read2count: R,
}

#[tonic::async_trait]
impl<K, P, R> CountSource for KeyToCnt<K, P, R>
where
    K: KeyToPath,
    P: PathToRead,
    R: ReadToCount<Read = P::Read>,
{
    type Key = K::Key;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<u64, Status> {
        let pat: PathBuf = self.key2path.key2path(key);
        let rd: P::Read = self.path2read.path_to_read(pat).await?;
        self.read2count.read2count(rd).await
    }
}

/// Creates a [`CountSource`] trait implementations.
pub fn cnt_src_key2cnt_new<K, P, R>(
    key2path: K,
    path2read: P,
    read2count: R,
) -> impl CountSource<Key = K::Key>
where
    K: KeyToPath,
    P: PathToRead,
    R: ReadToCount<Read = P::Read>,
{
    KeyToCnt {
        key2path,
        path2read,
        read2count,
    }
}
