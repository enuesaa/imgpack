use anyhow::{Ok, Result, bail};
use std::{fmt, fs, path::PathBuf};

use crate::fs::{ext::calc_ext, filesize::get_filesize, original::calc_originalpath};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ext {
    Png,
    Jpg,
}

pub struct Compressable {
    path: PathBuf,
    started: bool,
}

impl Compressable {
    pub fn from(path: PathBuf) -> Self {
        Self {
            path,
            started: false,
        }
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

    // Example: `a.png`
    pub fn initpath(&self) -> Result<PathBuf> {
        if self.started {
            bail!("compression started");
        };
        Ok(self.path.clone())
    }

    pub fn setup(&mut self) -> Result<()> {
        if self.started {
            bail!("compression started");
        };
        self.started = true;

        let from = self.path.clone();
        let to = self.inpath()?;
        Ok(fs::rename(from, to)?)
    }

    // Example: `a.original.png`
    pub fn inpath(&self) -> Result<PathBuf> {
        calc_originalpath(&self.path)
    }

    // Example: `a.png`
    pub fn outpath(&self) -> Result<PathBuf> {
        if !self.started {
            bail!("compression not started");
        };
        Ok(self.path.clone())
    }

    pub fn get_insize(&self) -> Result<f64> {
        Ok(get_filesize(&self.inpath()?)?)
    }

    pub fn get_outsize(&self) -> Result<f64> {
        Ok(get_filesize(&self.outpath()?)?)
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
