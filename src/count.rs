//! Traits, functions to get count info.

use tonic::Status;

/// Tries to get the count info by the key.
#[tonic::async_trait]
pub trait CountSource: Send + Sync {
    type Key: Send + Sync + Clone;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<u64, Status>;
}

/// Tries to get the raw count info by the key.
#[tonic::async_trait]
pub trait CountSourceRaw: Send + Sync {
    type Key: Send + Sync + Clone;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<Vec<u8>, Status>;
}

/// Tries to parse the raw count info to get the count info.
pub trait CountRawParser: Send + Sync {
    fn parse(&self, count_raw: &[u8]) -> Result<u64, Status>;
}

pub struct CntRawParserFn<F> {
    parser: F,
}

impl<F> CountRawParser for CntRawParserFn<F>
where
    F: Fn(&[u8]) -> Result<u64, Status> + Send + Sync,
{
    fn parse(&self, count_raw: &[u8]) -> Result<u64, Status> {
        (self.parser)(count_raw)
    }
}

/// Creates [`CountRawParser`] from the function `parser`.
pub fn cnt_raw_parser_new_fn<F>(parser: F) -> impl CountRawParser
where
    F: Fn(&[u8]) -> Result<u64, Status> + Send + Sync,
{
    CntRawParserFn { parser }
}

pub struct CntSrcRawParsed<R, P> {
    raw: R,
    parser: P,
}

#[tonic::async_trait]
impl<R, P> CountSource for CntSrcRawParsed<R, P>
where
    R: CountSourceRaw,
    P: CountRawParser,
{
    type Key = R::Key;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<u64, Status> {
        let raw: Vec<u8> = self.raw.get_count_by_key(key).await?;
        self.parser.parse(&raw)
    }
}

/// Creates [`CountSource`] from [`CountSourceRaw`] and [`CountRawParser`].
pub fn cnt_src_raw_parsed_new<R, P>(raw: R, parser: P) -> impl CountSource<Key = R::Key>
where
    R: CountSourceRaw,
    P: CountRawParser,
{
    CntSrcRawParsed { raw, parser }
}
