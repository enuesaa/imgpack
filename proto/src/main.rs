pub mod cli;
pub mod pack;

use std::path::Path;

use cli::{CLI, CLIParser};
use pack::pack::pack;

fn main() {
    CLI::parse();

    let filepath = "input.png";
    let path = Path::new(filepath);
    let filestem = path.file_stem().unwrap().to_string_lossy();
    let fileext = path.extension().unwrap().to_string_lossy();
    let outputfilepath = format!("{}.out.{}", filestem, fileext);

    let _ = pack(filepath.to_string(), outputfilepath);
}
