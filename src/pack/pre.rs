use anyhow::Result;
use std::fs;

use crate::fs::Compressable;

pub fn rename_original(file: &Compressable) -> Result<()> {
    let inpath = file.path();
    let outpath = file.inpath()?;

    fs::rename(inpath, outpath)?;
    Ok(())
}
