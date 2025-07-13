use crate::print_error;
use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

pub fn cat(args: &[String], current_dir: &PathBuf) -> i32 {
    if args.is_empty() {
        let stdin = io::stdin();
        for line_res in stdin.lock().lines() {
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
    }

    0
}
