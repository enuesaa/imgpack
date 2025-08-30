mod fs;
mod pack;

use anyhow::Result;
use fs::{Ext, list_compressables, Compressable};
use pack::{pack_jpg, pack_png, rename_original};
use std::{io::Write, path::PathBuf};

pub fn compress_dir<W: Write>(path: PathBuf, logger: &mut W) -> Result<()> {
    let files = list_compressables(&path)?;

    for file in files.iter() {
        writeln!(logger, "{}", file)?;
        rename_original(file)?;
        let ext = file.ext()?;

        match ext {
            Ext::Jpg => pack_jpg(file)?,
            Ext::Png => pack_png(file)?,
        };
        compare_filesize(file, logger)?;
    }
    Ok(())
}

pub fn compare_filesize<W: Write>(file: &Compressable, logger: &mut W) -> Result<()> {
    let out = file.get_out_filesize()?;
    let original = file.get_original_filesize()?;
    let compressed = ((1.0 - out as f64 /original as f64)*100.0) as u64;
    writeln!(logger, "original: {}, out: {} (-{}%)", original, out, compressed)?;
    Ok(())
}
