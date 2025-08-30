use anyhow::Result;

use crate::fs::Compressable;

pub fn compare_filesize(file: &Compressable) -> Result<String> {
    let outsize = file.get_outsize()?;
    let insize = file.get_insize()?;

    let compressed = ((1.0 - outsize as f64 / insize as f64) * 100.0) as u64;
    let text = format!(
        " ... size reduced by {}% ({}KB -> {}KB)",
        compressed, insize, outsize
    );
    Ok(text)
}
