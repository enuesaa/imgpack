mod fs;
mod pack;

use anyhow::Result;
use fs::{Ext, list_compressables, Compressable};
use pack::{pack_jpg, pack_png, rename_original};
use std::{io::Write, path::PathBuf};

pub fn compress_dir<W: Write>(path: PathBuf, mut logger: W) -> Result<()> {
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

pub fn compare_filesize<W: Write>(file: &Compressable, mut logger: W) -> Result<()> {
    let out = file.get_out_filesize()?;
    let original = file.get_original_filesize()?;
    writeln!(logger, "original: {}, out: {}", original, out)?;
    Ok(())
}
