use anyhow::Result;
use std::path::PathBuf;
use std::fs;

use crate::fs::compressable::Compressable;

pub fn list_compressables(path: &PathBuf) -> Result<Vec<Compressable>> {
    let files = list_files_in_dir(path)?;
    let targets = filter_compress_target(files);

    let compressables = targets
        .iter()
        .map(|a| Compressable::from(a.to_path_buf()))
        .collect();
    Ok(compressables)
}

fn list_files_in_dir(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut ret = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            ret.push(path);
        }
    }
    Ok(ret)
}

fn filter_compress_target(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .clone()
        .into_iter()
        .filter(|path| {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                name.ends_with(".png") || name.ends_with(".jpg") || name.ends_with(".jpeg")
            } else {
                false
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_compress_target() {
        let files = vec![
            PathBuf::from("a.png"),          // 圧縮済みなのでスキップ
            PathBuf::from("a.original.png"), // オリジナルファイルなのでスキップ
            PathBuf::from("b.jpg"),
            PathBuf::from("c.png"),
            PathBuf::from("d.original.png"), // オリジナルファイルなのでスキップ
        ];

        let res = filter_compress_target(files);

        let names: Vec<String> = res
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        assert_eq!(names, vec!["b.jpg", "c.png"]);
    }
}
