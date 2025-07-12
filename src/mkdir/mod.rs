use std::fs;
use std::path::PathBuf;

use crate::print_error;

pub fn mkdir(args: &[String], current_dir: &PathBuf) {
    if args.is_empty() {
        print_error("mkdir: missing operand");
        return;
    }

    for arg in args {
        let mut target = current_dir.clone();
        target.push(arg);

        if let Err(err) = fs::create_dir(&target) {
            print_error(&format!("mkdir: cannot create directory '{}': {}", arg, err));
        }
    }
}
