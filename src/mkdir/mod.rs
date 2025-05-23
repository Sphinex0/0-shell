use std::fs;
use std::path::PathBuf;

use crate::print_error;

pub fn mkdir(args: &[String], current_dir: &PathBuf) {
    let path_copy: &mut PathBuf = &mut current_dir.clone();
    let mut action_done: bool = false;

    for arg in args {
        let mut tmp = path_copy.clone();
        tmp.push(arg);

        if let Err(err) = fs::create_dir(tmp) {
            print_error(&err.to_string());
        }
        action_done = true;
    }
    if !action_done {
        print_error("mkdir: missing operand")
    }
}
