use anyhow::{Ok, Result, bail};
use std::{fmt, fs, path::PathBuf};

use crate::fs::{ext::calc_ext, original::calc_originalpath};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ext {
    Png,
    Jpg,
}

pub struct Compressable {
    path: PathBuf,
}

impl Compressable {
    pub fn from(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn ext(&self) -> Result<Ext> {
        let exts = calc_ext(&self.path)?;
        match exts.as_str() {
            "jpg" => Ok(Ext::Jpg),
            "jpeg" => Ok(Ext::Jpg),
            "png" => Ok(Ext::Png),
            _ => bail!("unsupported extension"),
        }
    }

    pub fn outpath(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn originalpath(&self) -> Result<PathBuf> {
        calc_originalpath(&self.path)
    }

    pub fn get_original_filesize(&self) -> Result<u64> {
        let path = self.originalpath()?;
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }

    pub fn get_out_filesize(&self) -> Result<u64> {
        let path = self.outpath();
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }
}

impl fmt::Display for Compressable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.path.file_name() {
            Some(name) => write!(f, "{}", name.to_string_lossy()),
            None => write!(f, "<unknown>"),
        }
    }
}
