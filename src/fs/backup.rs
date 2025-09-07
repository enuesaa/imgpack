use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time;

fn get_backup_dir() -> Result<PathBuf> {
    let home = env::home_dir().ok_or_else(|| anyhow!("failed to get home dir"))?;
    let dir = home.join(".imgpack");
    Ok(dir)
}

pub fn calc_backup_path(inpath: &PathBuf) -> Result<PathBuf> {
    let dir = get_backup_dir()?;
    let filename = inpath
        .file_name()
        .ok_or_else(|| anyhow!("failed to get filename"))?;
    let path = dir.join(filename);
    Ok(path)
}

fn is_backup_dir_exist() -> Result<bool> {
    let dir = get_backup_dir()?;
    Ok(fs::exists(dir)?)
}

fn mk_backup_dir() -> Result<()> {
    if is_backup_dir_exist()? {
        return Ok(());
    }
    let dir = get_backup_dir()?;
    let _ = fs::create_dir(dir)?;
    Ok(())
}

fn rm_backup_file(inpath: &PathBuf) -> Result<()> {
    let path = calc_backup_path(inpath)?;
    if fs::exists(path.clone())? {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn backup(inpath: &PathBuf) -> Result<()> {
    mk_backup_dir()?;
    rm_backup_file(inpath)?;
    let path = calc_backup_path(inpath)?;
    let _ = fs::copy(inpath, path)?;
    Ok(())
}

pub fn list_backuped_files() -> Result<Vec<PathBuf>> {
    if !is_backup_dir_exist()? {
        return Ok(vec![]);
    }
    let dir = get_backup_dir()?;
    let mut ret = Vec::new();

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
    let one_day_ago = time::SystemTime::now() - time::Duration::from_secs(24 * 60 * 60);

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
