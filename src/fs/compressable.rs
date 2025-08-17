use anyhow::{anyhow, Ok, Result};
use std::{fmt, path::PathBuf};

use crate::fs::{ext::calc_ext, out::calc_outpath};

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
            _ => Err(anyhow!("unsupported extension")),
        }
    }

    pub fn outpath(&self) -> Result<PathBuf> {
        calc_outpath(&self.path)
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

