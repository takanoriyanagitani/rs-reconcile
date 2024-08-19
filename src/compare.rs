use tonic::Status;

use crate::count::CountSource;

#[derive(Debug)]
pub struct CompareCount<K> {
    pub key: K,
    pub count_src: u64,
    pub count_tgt: u64,
}

/// The source of the [`CompareCount`].
#[tonic::async_trait]
pub trait CompareCountSource {
    type Key: Send + Sync + Clone;

    async fn compare(&self, key: &Self::Key) -> Result<CompareCount<Self::Key>, Status>;
}

pub struct CompareCntSrc<S, T> {
    src: S,
    tgt: T,
}

#[tonic::async_trait]
impl<S, T> CompareCountSource for CompareCntSrc<S, T>
where
    S: CountSource,
    T: CountSource<Key = S::Key>,
{
    type Key = S::Key;

    async fn compare(&self, key: &Self::Key) -> Result<CompareCount<Self::Key>, Status> {
        let src: u64 = self.src.get_count_by_key(key).await?;
        let tgt: u64 = self.tgt.get_count_by_key(key).await?;
        Ok(CompareCount {
            key: key.clone(),
            count_src: src,
            count_tgt: tgt,
        })
    }
}

/// Creates a [`CompareCountSource`] from the [`CountSource`](source, target).
pub fn compare_cnt_src_new<S, T>(src: S, tgt: T) -> impl CompareCountSource<Key = S::Key>
where
    S: CountSource,
    T: CountSource<Key = S::Key>,
{
    CompareCntSrc { src, tgt }
}
