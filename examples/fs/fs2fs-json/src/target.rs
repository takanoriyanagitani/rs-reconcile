use std::path::PathBuf;

use futures::stream::StreamExt;

use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;

use rs_reconcile::count::CountSource;

use rs_reconcile::source::fs::key2path::key2path_fn_new;
use rs_reconcile::source::fs::key2path::KeyToPath;

use rs_reconcile::target::fs::key2count::cnt_src_key2cnt_new;
use rs_reconcile::target::fs::path2read::path2read_fn_new;
use rs_reconcile::target::fs::path2read::PathToRead;
use rs_reconcile::target::fs::read2count::read2cnt_fn_new;
use rs_reconcile::target::fs::read2count::ReadToCount;

use crate::source::KeyInfo;

pub fn key2path_new(root: PathBuf) -> impl KeyToPath<Key = KeyInfo> {
    key2path_fn_new(move |key: &KeyInfo| {
        let dev = root.join(&key.uuid36);
        let ymd = dev.join(&key.ymd10);
        ymd.join("rows.jsonl")
    })
}

pub fn path2read_new() -> impl PathToRead<Read = tokio::fs::File> {
    path2read_fn_new(tokio::fs::File::open)
}

pub fn read2cnt_new() -> impl ReadToCount<Read = tokio::fs::File> {
    read2cnt_fn_new(|rd: tokio::fs::File| async move {
        let rdr = tokio::io::BufReader::new(rd);
        let lines = rdr.lines();
        let lstrm = LinesStream::new(lines);
        Ok(lstrm.count().await as u64)
    })
}

pub fn cnt_src_direct_new(root: PathBuf) -> impl CountSource<Key = KeyInfo> {
    cnt_src_key2cnt_new(key2path_new(root), path2read_new(), read2cnt_new())
}
