mod fs;
mod pack;

use anyhow::Result;
use fs::{Compressable, Ext, list_compressables};
use pack::{onafter_compare, onbefore_log, onbefore_rename, pack_jpg, pack_png};
use std::{io::Write, path::PathBuf};

pub fn compress_dir<W: Write>(path: PathBuf, logger: &mut W) -> Result<()> {
    let files = list_compressables(&path)?;
    for file in files.iter() {
        compress_file(file, logger)?;
    }
    Ok(())
}

pub fn compress_file<W: Write>(file: &Compressable, logger: &mut W) -> Result<()> {
    onbefore_log(file, logger)?;
    onbefore_rename(file)?;

    match file.ext()? {
        Ext::Jpg => pack_jpg(file)?,
        Ext::Png => pack_png(file)?,
    };

    onafter_compare(file, logger)?;
    Ok(())
}
