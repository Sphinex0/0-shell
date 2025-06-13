use std::path::PathBuf;

pub fn pwd(current_dir: &PathBuf) -> String {
    format!("\x1b[1;34m{}\x1b[0m", current_dir.display())
}
