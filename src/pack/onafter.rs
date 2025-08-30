use std::io::Write;

use anyhow::Result;

use crate::{context::Context, fs::Compressable};

pub fn onafter_compare(ctx: &mut Context, file: &Compressable) -> Result<()> {
    let outsize = file.get_outsize()?;
    let insize = file.get_insize()?;

    let compressed = ((1.0 - outsize / insize) * 100.0) as u64;
    write!(
        ctx.logger,
        "size reduced by {}% ({}KB -> {}KB)\n",
        compressed, insize, outsize
    )?;
    Ok(())
}
