use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn get_filesize(path: &PathBuf) -> Result<f64> {
    let metadata = fs::metadata(path)?;
    let bytes = metadata.len();
    let kb = (bytes as f64 / 1000.0).round();
    Ok(kb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filesize_testdata() {
        let path = PathBuf::from("testdata/lakemountain.png");
        let filesize = get_filesize(&path);
        assert_eq!(filesize.unwrap(), 3157.0);
        
        let path = PathBuf::from("testdata/lakemountain.jpg");
        let filesize = get_filesize(&path);
        assert_eq!(filesize.unwrap(), 632.0);
    }
}
