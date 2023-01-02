use std::{fs, path::Path};

pub fn filenames_in_path(path: &Path) -> Vec<String> {
    fs::read_dir(path).expect("Error reading directory!")
        .map(|f| f.unwrap().path().display().to_string())
        .collect()
}
