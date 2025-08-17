use anyhow::{anyhow, Ok, Result};
use std::{
    default,
    path::{Path, PathBuf},
};

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
