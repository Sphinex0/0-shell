use std::{fs, io::{self, BufRead}, path::PathBuf};
use crate::print_error;

pub fn cat(args: &[String], current_dir: &PathBuf) -> String {
    if args.is_empty() {
        let stdin = io::stdin();
        for line_res in stdin.lock().lines() {
            let line = match line_res {
                Ok(l) => l,
                Err(_) => break,
            };
            println!("{}", line);
        }
    } else {
        let mut result = String::new();
        for arg in args {
            let path = current_dir.join(arg);
            match fs::read_to_string(&path) {
                Ok(content) => result.push_str(&content),
                Err(e) => print_error(&format!("cat: {}: {}", arg, e)),
            }
        }
        result
    }
}
