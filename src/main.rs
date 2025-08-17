pub mod cli;
pub mod fs;
pub mod pack;

use anyhow::Result;
use cli::{CLIParser, CLI};
use fs::list::list;
use fs::compressable::Ext;
use pack::png::pack_png;
use pack::jpg::pack_jpg;

fn main() {
    CLI::parse();

    let _ = handle_compress();
}

fn handle_compress() -> Result<()> {
    let files = list()?;

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
