pub mod cli;
pub mod fs;
pub mod pack;

use anyhow::Result;
use cli::{CLIParser, CLI};
use fs::list::list;
use pack::pack::pack;

use crate::fs::out::calc_outpath;
use crate::pack::jpg::compress_jpeg;

fn main() {
    CLI::parse();
    let _ = compress_jpeg();

    // let _ = handle_compress();
}

fn handle_compress() -> Result<()> {
    let files = list()?;

    for file in files.iter() {
        println!("compress: {}", file.display());
        let outpath = &calc_outpath(file)?;
        pack(file, outpath)?;
    }

    Ok(())
}
