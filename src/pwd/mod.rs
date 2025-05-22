use std::path::PathBuf;

pub fn pwd(current_dir: &PathBuf) {
    println!("{}", current_dir.display());
}
