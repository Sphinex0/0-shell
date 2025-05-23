use std::fs;
use std::path::PathBuf;

pub fn mkdir(args: &[String], current_dir: &PathBuf) {
    let path_copy = &mut current_dir.clone();
    path_copy.push(&args[0]);
    fs::create_dir(path_copy).unwrap();
}
