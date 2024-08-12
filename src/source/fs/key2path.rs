use core::marker::PhantomData;

use std::path::PathBuf;

pub trait KeyToPath: Sync + Send {
    type Key: Send + Sync + Clone;

    fn key2path(&self, key: &Self::Key) -> PathBuf;
}

pub struct KeyToPathFn<F, K> {
    key2path: F,
    ph: PhantomData<K>,
}

impl<F, K> KeyToPath for KeyToPathFn<F, K>
where
    F: Fn(&K) -> PathBuf + Sync + Send,
    K: Send + Sync + Clone,
{
    type Key = K;

    fn key2path(&self, key: &Self::Key) -> PathBuf {
        (self.key2path)(key)
    }
}

pub fn key2path_fn_new<F, K>(key2path: F) -> impl KeyToPath<Key = K>
where
    F: Fn(&K) -> PathBuf + Sync + Send,
    K: Send + Sync + Clone,
{
    KeyToPathFn {
        key2path,
        ph: PhantomData,
    }
}
