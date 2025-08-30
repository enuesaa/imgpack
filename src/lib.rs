mod fs;
mod pack;

use anyhow::Result;
use fs::{Compressable, Ext, list_compressables};
use pack::{compare_filesize, pack_jpg, pack_png, rename_original};
use std::{io::Write, path::PathBuf};

pub fn compress_dir<W: Write>(path: PathBuf, logger: &mut W) -> Result<()> {
    let files = list_compressables(&path)?;
    for file in files.iter() {
        compress_file(file, logger)?;
    }
    Ok(())
}

pub fn compress_file<W: Write>(file: &Compressable, logger: &mut W) -> Result<()> {
    write!(logger, "{}", file)?;
    rename_original(file)?;
    let ext = file.ext()?;

    match ext {
        Ext::Jpg => pack_jpg(file)?,
        Ext::Png => pack_png(file)?,
    };
    write!(logger, "{}\n", compare_filesize(file)?)?;
    Ok(())
}
