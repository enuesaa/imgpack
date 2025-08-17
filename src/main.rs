pub mod cli;
pub mod fs;
pub mod pack;

use core::panic;

use anyhow::Result;
use cli::{CLIParser, CLI};
use fs::list::list;
use pack::pack::pack;

use crate::fs::compressable::Ext;
use crate::pack::jpg::compress_jpg;

fn main() {
    CLI::parse();

    let _ = handle_compress();
}

fn handle_compress() -> Result<()> {
    let files = list()?;

    for file in files.iter() {
        // println!("compress: {}", file.display());

        let ext = file.ext()?;

        match ext {
            Ext::Jpg => compress_jpg(file)?,
            Ext::Png => pack(file)?,
        };
    }
    Ok(())
}
