use std::path::PathBuf;

pub use clap::Parser as CLIParser;

/// A CLI tool to compress png/jpg images.
#[derive(CLIParser)]
#[command(bin_name = "imgpack", version = "v0.1.0")]
pub struct CLI {
    /// directory to compress
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,
}
