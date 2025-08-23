mod fs;
mod pack;

use std::path::PathBuf;

use anyhow::Result;
use fs::{Ext, list};
use pack::{pack_jpg, pack_png};

pub fn handle_compress(path: PathBuf) -> Result<()> {
    let files = list(&path)?;

    for file in files.iter() {
        println!("compress: {}", file);
        let ext = file.ext()?;

        match ext {
            Ext::Jpg => pack_jpg(file)?,
            Ext::Png => pack_png(file)?,
        };
    }
    Ok(())
}