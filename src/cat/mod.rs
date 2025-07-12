use std::{fs, io::{self, stdin, Write}, path::PathBuf};
use crate::print_error;

pub fn cat(args: &[String], current_dir: &PathBuf) -> String {
    let mut result = String::new();

    if args.is_empty() {
        let mut line = String::new();
        let stdin = stdin();
        let mut stdout = io::stdout();

        loop {
            line.clear();
            let read = stdin.read_line(&mut line).unwrap();
            if read == 0 {
                break;
            }
            result.push_str(&line);
            write!(stdout, "{}", line).unwrap();
            stdout.flush().unwrap();
        }

        return result;
    }

    for arg in args {
        let path = current_dir.join(arg);
        match fs::read_to_string(&path) {
            Ok(content) => result.push_str(&content),
            Err(e) => print_error(&format!("cat: {}: {}", arg, e)),
        }
    }

    result
}
