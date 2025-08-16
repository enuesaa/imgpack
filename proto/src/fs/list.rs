use std::fs;

use anyhow::Result;
use std::path::PathBuf;

pub fn list() -> Result<Vec<PathBuf>> {
    let files = list_files_in_current_dir()?;
    let targets = filter_compress_target(files);

    Ok(targets)
}

fn list_files_in_current_dir() -> Result<Vec<PathBuf>> {
    let mut ret = Vec::new();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            ret.push(path);
        }
    }
    Ok(ret)
}

fn filter_compress_target(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|path| {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                name.ends_with(".png") && !name.contains(".out.")
            } else {
                false
            }
        })
        .collect()
}
