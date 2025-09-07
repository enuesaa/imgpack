use anyhow::{Ok, Result, bail};
use std::path::PathBuf;
use std::{fmt, fs};

use crate::fs::backup;
use crate::fs::ext::calc_ext;
use crate::fs::filesize::get_filesize;

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

    pub fn setup_compression(&mut self) -> Result<()> {
        if self.started {
            bail!("compression started");
        };
        self.started = true;

        backup::backup(&self.path.clone())?;
        fs::remove_file(&self.path.clone())?;
        Ok(())
    }

    fn backuppath(&self) -> Result<PathBuf> {
        backup::calc_backup_path(&self.path)
    }

    // Example: `a.png`
    pub fn inpath(&self) -> Result<PathBuf> {
        if self.started {
            return Ok(self.backuppath()?);
        };
        Ok(self.path.clone())
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
