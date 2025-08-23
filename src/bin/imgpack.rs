use clap::{Parser, crate_version};
use std::path::PathBuf;

use imgpack::compress_dir;

/// A CLI tool to compress png/jpg images.
#[derive(Parser)]
#[command(bin_name = "imgpack")]
pub struct CLI {
    /// directory to compress
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Print version
    #[arg(short = 'v', long = "version", global = true)]
    pub version: bool,
}

fn main() {
    let args = CLI::parse();

    if args.version {
        println!("{}", crate_version!());
        return;
    }

    if let Err(e) = compress_dir(args.path, &mut std::io::stdout()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
