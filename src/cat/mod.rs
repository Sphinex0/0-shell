use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn cat(args: &[&str], current_dir: &PathBuf) {
    let path_copy = &mut current_dir.clone();
    path_copy.push(args[0]);

    println!("{}", std::fs::read_to_string(path_copy).unwrap())
}
