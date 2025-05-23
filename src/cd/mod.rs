use std::{env::home_dir, path::PathBuf};

use crate::print_error;

pub fn cd(tab: &[String], current_dir: &mut PathBuf, history: &mut PathBuf) {
    if tab.len() == 0 {
        current_dir.push(&history);
        match home_dir() {
            Some(p) => current_dir.push(p),
            None => {}
        }
        return;
    } else {
        let path = tab[0].as_str();
        if path != "-" {
            history.push(&current_dir);
        }
        match path {
            "/" => current_dir.push("/"),
            "~" => {
                match home_dir() {
                    Some(p) => current_dir.push(p),
                    None => {}
                }
                return;
            }
            "-" => {
                current_dir.push(&history);
            }
            _ => {
                if &path[0..1] == "/" {
                    let mut copy_current_dir = current_dir.clone();
                    copy_current_dir.push(path);
                    match copy_current_dir.read_dir() {
                        Ok(_) => current_dir.push(path),
                        Err(_) => {
                            print_error(&("cd: no such file or directory: ".to_string() + path))
                        }
                    }
                } else {
                    let table = path.split("/").collect::<Vec<_>>();
                    let mut copy_current_dir = current_dir.clone();
                    for p in table {
                        match p {
                            "." => {}
                            ".." => {
                                copy_current_dir.pop();
                            }
                            _ => {
                                copy_current_dir.push(p);
                            }
                        }
                    }
                    match copy_current_dir.read_dir() {
                        Ok(_) => current_dir.push(copy_current_dir),
                        Err(_) => {
                            print_error(&("cd: no such file or directory: ".to_string() + path))
                        }
                    }
                }
            }
        }
    }
}
