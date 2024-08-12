use std::path::PathBuf;

use rs_reconcile::count::cnt_raw_parser_new_fn;
use rs_reconcile::count::cnt_src_raw_parsed_new;

use rs_reconcile::count::CountRawParser;
use rs_reconcile::count::CountSource;
use rs_reconcile::count::CountSourceRaw;

use tonic::Status;

#[derive(serde::Deserialize)]
pub struct StatInfo {
    pub cnt: u64,
}

#[derive(Debug, Clone)]
pub struct KeyInfo {
    /// uuid string, 36-bytes(e.g, cafef00d-dead-beaf-face-864299792458)
    pub uuid36: String,

    /// ymd string, 10-bytes(e.g, 2024-08-12)
    pub ymd10: String,
}

pub fn cnt_raw_parser_stat_info() -> impl CountRawParser {
    cnt_raw_parser_new_fn(|raw: &[u8]| {
        let sinf: StatInfo = serde_json::from_slice(raw)
            .map_err(|e| Status::invalid_argument(format!("unable to parse: {e}")))?;
        Ok(sinf.cnt)
    })
}

pub struct CntSrcRaw {
    root: PathBuf,
}

#[tonic::async_trait]
impl CountSourceRaw for CntSrcRaw {
    type Key = KeyInfo;

    async fn get_count_by_key(&self, key: &Self::Key) -> Result<Vec<u8>, Status> {
        let dev = self.root.join(&key.uuid36);
        let ymd = dev.join(&key.ymd10);
        let stat = ymd.join("stat.json");
        let raw: Vec<u8> = tokio::fs::read(&stat).await.map_err(|e| {
            Status::internal(format!("unable to read the stat info file({stat:#?}): {e}"))
        })?;
        Ok(raw)
    }
}

pub fn cnt_src_stat_info_new(root: PathBuf) -> impl CountSource<Key = KeyInfo> {
    let parser = cnt_raw_parser_stat_info();
    let raw_source = CntSrcRaw { root };
    cnt_src_raw_parsed_new(raw_source, parser)
}
