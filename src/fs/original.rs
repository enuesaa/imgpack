use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std;
use std::env;

pub fn get_backup_dir() -> Result<PathBuf> {
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

pub fn calc_originalpath(file: &PathBuf) -> Result<PathBuf> {
    let stem = file
        .file_stem()
        .ok_or_else(|| anyhow!("failed to parse file stem"))?
        .to_string_lossy();
    let ext = file
        .extension()
        .ok_or_else(|| anyhow!("failed to parse file ext"))?
        .to_string_lossy();

    let mut path = file
        .parent()
        .ok_or_else(|| anyhow!("failed to get parent dir"))?
        .to_path_buf();
    path.push(format!("{}.original.{}", stem, ext));

    Ok(path)
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

    #[test]
    fn test_calc_originalpath() {
        let input = PathBuf::from("/some/path/file.txt");
        let expected = PathBuf::from("/some/path/file.original.txt");
        assert_eq!(calc_originalpath(&input).unwrap(), expected);
    }

    #[test]
    fn test_calc_originalpath_no_extension() {
        let input = PathBuf::from("/some/path/file");
        assert!(calc_originalpath(&input).is_err());
    }
}
