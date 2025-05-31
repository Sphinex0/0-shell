use std::path::PathBuf;

pub fn pwd(current_dir: &PathBuf)-> String {
    // println!("{}", current_dir.display());
    current_dir.display().to_string()
}
