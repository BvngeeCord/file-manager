use std::{fs, path::Path};

pub fn filenames_in_path(path: &Path) -> Vec<String> {
    let mut filenames: Vec<String> = fs::read_dir(path).expect("Error reading directory!")
        .map(|f| f.unwrap().path().display().to_string())
        .collect();
    filenames.sort_by(|a, b| b.cmp(a));

    filenames
}
