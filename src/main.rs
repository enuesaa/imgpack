mod cli;
mod fs;
mod pack;

use anyhow::Result;
use cli::{CLIParser, CLI};
use fs::{list, Ext};
use pack::{pack_jpg, pack_png};

fn main() {
    let cli = CLI::parse();

    if let Err(e) = handle_compress(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_compress(cli: CLI) -> Result<()> {
    let files = list(&cli.path)?;

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
