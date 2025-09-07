use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

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

pub fn list_backuped_files() -> Result<Vec<PathBuf>> {
    let dir = get_backup_dir()?;
    let mut ret = Vec::new();

    if let Ok(is) = fs::exists(dir.clone()) && !is {
        return Ok(ret);
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            ret.push(path);
        }
    }
    Ok(ret)
}

pub fn remove_old_backups() -> Result<()> {
    let dir = get_backup_dir()?;
    let one_day_ago = SystemTime::now() - Duration::from_secs(24 * 60 * 60);

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let created = metadata.created()?;
            if created < one_day_ago {
                fs::remove_file(path)?;
            }
        }
    }
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
