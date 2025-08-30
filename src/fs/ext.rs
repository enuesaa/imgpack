use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn calc_ext(file: &PathBuf) -> Result<String> {
    let ext = file
        .extension()
        .ok_or_else(|| anyhow!("failed to parse file ext"))?
        .to_string_lossy()
        .to_string();
    Ok(ext)
}
