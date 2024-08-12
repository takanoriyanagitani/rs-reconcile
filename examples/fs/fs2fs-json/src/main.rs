use std::path::PathBuf;
use std::process::ExitCode;

use tonic::Status;

use rs_reconcile::compare::compare_cnt_src_new;
use rs_reconcile::compare::CompareCount;
use rs_reconcile::compare::CompareCountSource;

use crate::source::KeyInfo;

async fn sub() -> Result<(), Status> {
    let root: PathBuf = "./sample.d/source.d".into();
    let src_cnt_provider = crate::source::cnt_src_stat_info_new(root);

    let root: PathBuf = "./sample.d/target.d".into();
    let tgt_cnt_provider = crate::target::cnt_src_direct_new(root);

    let cmp_src = compare_cnt_src_new(src_cnt_provider, tgt_cnt_provider);

    let keys = vec![
        KeyInfo {
            uuid36: "cafef00d-dead-beaf-face-864299792458".into(),
            ymd10: "2024-08-10".into(),
        },
        KeyInfo {
            uuid36: "cafef00d-dead-beaf-face-864299792458".into(),
            ymd10: "2024-08-11".into(),
        },
        KeyInfo {
            uuid36: "cafef00d-dead-beaf-face-864299792458".into(),
            ymd10: "2024-08-12".into(),
        },
    ];

    for key in keys {
        let cmp_rslt: CompareCount<_> = cmp_src.compare(&key).await?;

        println!("{cmp_rslt:#?}");
    }

    Ok(())
}

mod source;
mod target;

#[tokio::main]
async fn main() -> ExitCode {
    sub().await.map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
