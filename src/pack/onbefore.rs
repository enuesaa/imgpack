use anyhow::Result;
use std::io::Write;

use crate::{context::Context, fs::Compressable};

pub fn onbefore_log(ctx: &mut Context, file: &Compressable) -> Result<()> {
    Ok(write!(ctx.logger, "{} ... ", file)?)
}

pub fn onbefore_setup(file: &mut Compressable) -> Result<()> {
    Ok(file.setup_compression()?)
}
