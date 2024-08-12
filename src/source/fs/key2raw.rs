use std::path::PathBuf;

use tonic::Status;

use crate::count::CountSourceRaw;

use super::key2path::KeyToPath;
use super::path2raw::PathToRaw;

pub struct KeyToRawFs<K, P> {
    key2path: K,
    path2raw: P,
}

#[tonic::async_trait]
impl<K, P> CountSourceRaw for KeyToRawFs<K, P>
where
    K: KeyToPath,
    P: PathToRaw,
{
    type Key = K::Key;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<Vec<u8>, Status> {
        let pat: PathBuf = self.key2path.key2path(key);
        self.path2raw.path_to_raw(pat).await
    }
}

pub fn key2raw_fs_new<K, P>(key2path: K, path2raw: P) -> impl CountSourceRaw<Key = K::Key>
where
    K: KeyToPath,
    P: PathToRaw,
{
    KeyToRawFs { key2path, path2raw }
}
