use std::path::PathBuf;

use anyhow::{anyhow, Result};

pub fn calc_outpath(file: &PathBuf) -> Result<PathBuf> {
    let stem = file
        .file_stem()
        .ok_or_else(|| anyhow!("failed to parse file stem"))?
        .to_string_lossy();
    let ext = file
        .file_stem()
        .ok_or_else(|| anyhow!("failed to parse file ext"))?
        .to_string_lossy();

    let mut outpath = file
        .parent()
        .ok_or_else(|| anyhow!("failed to get parent dir"))?
        .to_path_buf();
    outpath.push(format!("{}.out.{}", stem, ext));

    Ok(outpath)
}
