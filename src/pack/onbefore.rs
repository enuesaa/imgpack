use anyhow::Result;
use std::{fs, io::Write};

use crate::{context::Context, fs::Compressable};

pub fn onbefore_log(ctx: &mut Context, file: &Compressable) -> Result<()> {
    Ok(write!(ctx.logger, "{} ... ", file)?)
}

pub fn onbefore_rename(file: &Compressable) -> Result<()> {
    let inpath = file.path();
    let outpath = file.inpath()?;

    fs::rename(inpath, outpath)?;
    Ok(())
}
