use std::fs;
use std::path::PathBuf;

use crate::print_error;
pub fn rm(args: &[String], current_dir: &PathBuf) {
    let path_copy: &mut PathBuf = &mut current_dir.clone();
    let mut action_done: bool = false;
    for arg in args {
        if arg != &"-r" {
            let mut tmp = path_copy.clone();
            tmp.push(arg);

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
            action_done = true;
        }
    }
    if !action_done {
        print_error("rm: missing operand")
    }
}
