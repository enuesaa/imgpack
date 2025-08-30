use clap::{Parser, crate_version};
use std::path::PathBuf;

use imgpack::{compress_dir, create_cli_context};

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
    let mut ctx = create_cli_context();

    if let Err(e) = compress_dir(&mut ctx, args.path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
