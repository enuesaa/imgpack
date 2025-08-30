use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn get_filesize(path: &PathBuf) -> Result<f64> {
    let metadata = fs::metadata(path)?;
    let bytes = metadata.len();
    let kb = (bytes as f64 / 1000.0).round();
    Ok(kb)
}
