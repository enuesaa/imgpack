pub mod cli;
pub mod pack;
pub mod fs;

use anyhow::Result;
use cli::{CLI, CLIParser};
use fs::list::list;
use pack::pack::pack;

use crate::fs::out::calc_outpath;

fn main() {
    CLI::parse();

    let _ = handle_compress();
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
