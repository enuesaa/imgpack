mod fs;
mod pack;

use std::{io::Write, path::PathBuf};

use anyhow::Result;
use fs::{Ext, list_compressables};
use pack::{pack_jpg, pack_png};

pub fn compress_dir<W: Write>(path: PathBuf, mut logger: W) -> Result<()> {
    let files = list_compressables(&path)?;

    for file in files.iter() {
        writeln!(logger, "{}", file)?;
        let ext = file.ext()?;

        match ext {
            Ext::Jpg => pack_jpg(file)?,
            Ext::Png => pack_png(file)?,
        };
    }
    Ok(())
}
