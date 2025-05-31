use std::path::PathBuf;

use crate::print_error;

pub fn cat(args: &[String], current_dir: &PathBuf) -> String {
    let path_copy: &mut PathBuf = &mut current_dir.clone();
    let mut action_done: bool = false;

    for arg in args {
        let mut tmp = path_copy.clone();
        tmp.push(arg);

        match std::fs::read_to_string(tmp) {
            Ok(content) => {
                return content;
            }
            Err(err) => print_error(&format!("{arg}: {err}")),
        }

        action_done = true;
    }
    if !action_done {
        print_error("cat: missing operand");
    }
    "".to_string()
}
