use std::{fs, io::{stdin, Write}, path::PathBuf};
use crate::print_error;

pub fn cat(args: &[String], current_dir: &PathBuf) -> String {
    let mut result = String::new();
    let mut file_found = false;

    for arg in args {
        let mut full_path = current_dir.clone();
        full_path.push(arg);

        match fs::read_to_string(&full_path) {
            Ok(content) => {
                result.push_str(&content);
                file_found = true;
            }
            Err(err) => print_error(&format!("{arg}: {err}")),
        }
    }

    if file_found {
        return result;
    }

    loop {
        let mut input_tmp = String::new();
        std::io::stdout().flush().unwrap();
        let size = stdin().read_line(&mut input_tmp).unwrap();
        if size == 0 {
            break;
        }
        result.push_str(&input_tmp);
    }

    result
}
