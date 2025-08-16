pub mod cli;
pub mod pack;

use cli::{CLI, CLIParser};
use pack::pack::pack;

fn main() {
    CLI::parse();
    println!("Hello!");

    let _ = pack();
}
