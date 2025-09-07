use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::fs;
use std::env;

fn get_backup_dir() -> Result<PathBuf> {
    let home = env::home_dir().ok_or_else(|| anyhow!("failed to get home dir"))?;
    let dir = home.join(".imgpack");
    Ok(dir)
}

pub fn calc_backup_path(file: &PathBuf) -> Result<PathBuf> {
    let dir = get_backup_dir()?;
    let filename = file
        .file_name()
        .ok_or_else(|| anyhow!("failed to get filename"))?;
    let path = dir.join(filename);
    Ok(path)
}

fn mk_backup_dir() -> Result<()> {
    let dir = get_backup_dir()?;
    let _ = fs::create_dir(dir)?;
    Ok(())
}

pub fn backup(inpath: &PathBuf) -> Result<()> {
    mk_backup_dir()?;
    let backup_path = calc_backup_path(inpath)?;
    let _ = fs::copy(inpath, backup_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_backup_path() {
        temp_env::with_var("HOME", Some("/home"), || {
            let input = PathBuf::from("/some/path/file.txt");
            let expected = PathBuf::from("/home/.imgpack/file.txt");
            assert_eq!(calc_backup_path(&input).unwrap(), expected);
        });
    }
}
