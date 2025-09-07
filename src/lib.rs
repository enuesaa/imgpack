pub mod context;
mod fs;
mod pack;

use anyhow::Result;
use context::Context;
pub use context::create_cli_context;
use std::path::PathBuf;

pub fn compress_dir(ctx: &mut Context, path: PathBuf) -> Result<()> {
    let mut files = fs::list_compressables(&path)?;
    for file in files.iter_mut() {
        compress_file(ctx, file)?;
    }
    fs::rm_old_backups()?;
    Ok(())
}

pub fn compress_file(ctx: &mut Context, file: &mut fs::Compressable) -> Result<()> {
    pack::onbefore_log(ctx, file)?;
    pack::onbefore_setup(file)?;

    match file.ext()? {
        fs::Ext::Jpg => pack::pack_jpg(file)?,
        fs::Ext::Png => pack::pack_png(file)?,
    };

    pack::onafter_compare(ctx, file)?;
    Ok(())
}
