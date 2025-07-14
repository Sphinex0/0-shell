use std::fs;
use std::path::PathBuf;

use crate::print_error;

pub fn rm(args: &[String], current_dir: &PathBuf) -> i32 {
    let mut recursive = false;
    let mut paths = vec![];
    // Parse arguments to separate flags and paths
    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            paths.push(arg);
        }
    }
    // Check if any paths were provided
    if paths.is_empty() {
        print_error("rm: missing operand");
        return 1;
    }
    for arg in paths {
        let mut tmp = current_dir.clone();
        for part in arg.split('/') {
            match part {
                "." => {}
                ".." => { tmp.pop(); }
                _ => { tmp.push(part); }
            }
        }
        if tmp.is_dir() {
            if recursive {
                if let Err(err) = fs::remove_dir_all(&tmp) {
                    print_error(&format!("{arg}: {err}"));
                }
            } else {
                print_error(&format!("rm: cannot remove '{arg}': Is a directory"));
            }
        } else {
            if let Err(err) = fs::remove_file(&tmp) {
                print_error(&format!("{arg}: {err}"));
            }
        }
    }
    0
}