use crate::print_error;
use std::{
    fs, io, path::PathBuf
};

pub fn cat(args: &[String], current_dir: &PathBuf) -> i32 {
    // If no arguments are provided, read and print from stdin
    if args.is_empty() {
        let stdin = io::stdin();
        for line_res in stdin.lines() {
            let line = match line_res {
                Ok(l) => l,
                Err(_) => {
                    return 1;
                }
            };
            println!("{}", line);
        }
    } else {
        let mut result = String::new();
        // Read and concatenate content of all provided files
        for arg in args {
            let path = current_dir.join(arg);
            match fs::read_to_string(&path) {
                Ok(content) => result.push_str(&content),
                Err(e) => {
                    print_error(&format!("cat: {}: {}", arg, e));
                    return 1;
                },
            }
        }
        print!("{}", result);
    }
    0
}