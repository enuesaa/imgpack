pub mod pack;

use clap::Parser;

/// A CLI tool to compress png images.
#[derive(Parser)]
#[command(bin_name = "imgpack", version = "v0.0.1")]
struct Args {}

fn main() {
    Args::parse();
    println!("Hello!");
}
