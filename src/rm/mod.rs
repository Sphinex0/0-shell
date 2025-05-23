use std::fs;
use std::path::PathBuf;

use crate::print_error;
pub fn rm(args: &[String], current_dir: &PathBuf) {
    let path_copy: &mut PathBuf = &mut current_dir.clone();
    let mut file_deleted: bool = false;
    for arg in args {
        if arg != &"-r" {
            let mut tmp = PathBuf::from(arg);
            if !arg.starts_with("/") {
                tmp = path_copy.clone();
                tmp.push(arg);
            }
            match tmp.read_dir() {
                Ok(_files) => {
                    if let Err(err) = fs::remove_dir_all(tmp) {
                        print_error(&err.to_string());
                    }
                }
                Err(_err) => {
                    if let Err(err) = fs::remove_file(tmp) {
                        print_error(&err.to_string());
                    }
                }
            }
            file_deleted = true;
        }
    }
    if !file_deleted {
        print_error("rm: missing operand")
    }
}
