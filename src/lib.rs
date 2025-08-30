pub mod context;
mod fs;
mod pack;

use anyhow::Result;
use context::Context;
pub use context::create_cli_context;
use fs::{Compressable, Ext, list_compressables};
use pack::{onafter_compare, onbefore_log, onbefore_setup, pack_jpg, pack_png};
use std::path::PathBuf;

pub fn compress_dir(ctx: &mut Context, path: PathBuf) -> Result<()> {
    let mut files = list_compressables(&path)?;
    for file in files.iter_mut() {
        compress_file(ctx, file)?;
    }
    Ok(())
}

pub fn compress_file(ctx: &mut Context, file: &mut Compressable) -> Result<()> {
    onbefore_log(ctx, file)?;
    onbefore_setup(file)?;

    match file.ext()? {
        Ext::Jpg => pack_jpg(file)?,
        Ext::Png => pack_png(file)?,
    };

    onafter_compare(ctx, file)?;
    Ok(())
}
