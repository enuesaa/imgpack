use std::io::Write;

use anyhow::Result;

use crate::fs::Compressable;

pub fn onafter_compare<W: Write>(file: &Compressable, logger: &mut W) -> Result<()> {
    let outsize = file.get_outsize()?;
    let insize = file.get_insize()?;

    let compressed = ((1.0 - outsize / insize) * 100.0) as u64;
    write!(
        logger,
        "size reduced by {}% ({}KB -> {}KB)\n",
        compressed, insize, outsize
    )?;
    Ok(())
}
