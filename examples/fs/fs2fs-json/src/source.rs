use std::path::PathBuf;

use rs_reconcile::count::cnt_raw_parser_new_fn;
use rs_reconcile::count::cnt_src_raw_parsed_new;

use rs_reconcile::count::CountRawParser;
use rs_reconcile::count::CountSource;
use rs_reconcile::count::CountSourceRaw;

use rs_reconcile::source::fs::path2raw::path2raw_new_fn;
use rs_reconcile::source::fs::path2raw::PathToRaw;

use rs_reconcile::source::fs::key2path::key2path_fn_new;
use rs_reconcile::source::fs::key2path::KeyToPath;

use rs_reconcile::source::fs::key2raw::key2raw_fs_new;

use tonic::Status;

pub fn path2raw_tokio_new() -> impl PathToRaw {
    path2raw_new_fn(tokio::fs::read)
}

pub fn key2path_new(root: PathBuf) -> impl KeyToPath<Key = KeyInfo> {
    key2path_fn_new(move |key: &KeyInfo| {
        let dev = root.join(&key.uuid36);
        let ymd = dev.join(&key.ymd10);
        ymd.join("stat.json")
    })
}

pub fn cnt_src_raw_new(root: PathBuf) -> impl CountSourceRaw<Key = KeyInfo> {
    key2raw_fs_new(key2path_new(root), path2raw_tokio_new())
}

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

pub fn cnt_src_stat_info_new(root: PathBuf) -> impl CountSource<Key = KeyInfo> {
    let parser = cnt_raw_parser_stat_info();
    let raw_source = cnt_src_raw_new(root);
    cnt_src_raw_parsed_new(raw_source, parser)
}
