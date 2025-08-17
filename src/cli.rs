pub use clap::Parser as CLIParser;

/// A CLI tool to compress png images.
#[derive(CLIParser)]
#[command(bin_name = "imgpack", version = "v0.0.1")]
pub struct CLI {}
