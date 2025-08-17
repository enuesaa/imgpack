use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn calc_outpath(file: &PathBuf) -> Result<PathBuf> {
    let stem = file
        .file_stem()
        .ok_or_else(|| anyhow!("failed to parse file stem"))?
        .to_string_lossy();
    let ext = file
        .extension()
        .ok_or_else(|| anyhow!("failed to parse file ext"))?
        .to_string_lossy();

    let mut outpath = file
        .parent()
        .ok_or_else(|| anyhow!("failed to get parent dir"))?
        .to_path_buf();
    outpath.push(format!("{}.out.{}", stem, ext));

    Ok(outpath)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_outpath() {
        let input = PathBuf::from("/some/path/file.txt");
        let expected = PathBuf::from("/some/path/file.out.txt");
        assert_eq!(calc_outpath(&input).unwrap(), expected);
    }

    #[test]
    fn test_calc_outpath_no_extension() {
        let input = PathBuf::from("/some/path/file");
        assert!(calc_outpath(&input).is_err());
    }
}
