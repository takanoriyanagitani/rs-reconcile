use std::path::PathBuf;

use futures::stream::StreamExt;

use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;

use rs_reconcile::count::CountSource;

use tonic::Status;

use crate::source::KeyInfo;

pub struct CntSrcDirect {
    root: PathBuf,
}

#[tonic::async_trait]
impl CountSource for CntSrcDirect {
    type Key = KeyInfo;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<u64, Status> {
        let dev = self.root.join(&key.uuid36);
        let ymd = dev.join(&key.ymd10);
        let rows = ymd.join("rows.jsonl");
        let fil = tokio::fs::File::open(rows)
            .await
            .map_err(|e| Status::internal(format!("unable to open the rows file: {e}")))?;
        let rdr = tokio::io::BufReader::new(fil);
        let lines = rdr.lines();
        let lstrm = LinesStream::new(lines);
        let cnt: usize = lstrm.count().await;
        Ok(cnt as u64)
    }
}

pub fn cnt_src_direct_new(root: PathBuf) -> impl CountSource<Key = KeyInfo> {
    CntSrcDirect { root }
}
