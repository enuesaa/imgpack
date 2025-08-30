use anyhow::Result;
use std::{fs, io::Write};

use crate::fs::Compressable;

pub fn onbefore_log<W: Write>(file: &Compressable, logger: &mut W) -> Result<()> {
    Ok(write!(logger, "{} ... ", file)?)
}

pub fn onbefore_rename(file: &Compressable) -> Result<()> {
    let inpath = file.path();
    let outpath = file.inpath()?;

    fs::rename(inpath, outpath)?;
    Ok(())
}
