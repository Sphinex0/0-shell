use std::{io::{stdin, Write}, path::PathBuf};

use crate::print_error;

pub fn cat(args: &[String], current_dir: &PathBuf) -> String {
    let path_copy: &mut PathBuf = &mut current_dir.clone();
    let mut action_done: bool = false;

    for arg in args {
        let mut tmp: PathBuf = path_copy.clone();
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
        loop {
            let mut input_tmp = String::new();
            std::io::stdout().flush().unwrap();
            let size = stdin().read_line(&mut input_tmp).unwrap();
            if size == 0 {
                // println!();
                break;
            }
            print!("{input_tmp}")
        }
        // print_error("cat: missing operand");
    }
    "".to_string()
}
