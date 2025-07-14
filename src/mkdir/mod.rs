use std::fs;
use std::path::PathBuf;
use crate::print_error;

pub fn mkdir(args: &[String], current_dir: &PathBuf) -> i32 {
    // Check if any directory arguments are provided
    if args.is_empty() {
        print_error("mkdir: missing operand");
        return 1;
    }

    for arg in args {
        let path = PathBuf::from(arg);
        // Construct target path, handling both absolute and relative paths
        let target = if path.is_absolute() {
            path
        } else {
            current_dir.join(path)
        };

        if let Err(e) = fs::create_dir(&target) {
            print_error(&format!("mkdir: cannot create directory '{}': {}", arg, e));
        }
    }
    0
}