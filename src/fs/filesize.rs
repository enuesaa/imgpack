use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn get_filesize(path: &PathBuf) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    let bytes = metadata.len();
    let kb = (bytes as f64 / 1000.0).round() as u64;
    Ok(kb)
}
