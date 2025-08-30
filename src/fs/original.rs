use anyhow::{anyhow, Result};
use std::path::PathBuf;

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
