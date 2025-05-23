use std::{env::home_dir, path::PathBuf};

pub fn cd(tab: &[String], current_dir: &mut PathBuf, history: &mut PathBuf) {
    if tab.len() == 0 {
        *history = current_dir.clone();
        *current_dir = PathBuf::from("/");
    } else {
        let path = tab[0].as_str();
        if path != "-" {
            *history = current_dir.clone();
        }
        match path {
            "/" => *current_dir = PathBuf::from("/"),
            "~" => {
                match home_dir() {
                    Some(p) => *current_dir = p,
                    None => {}
                }
                return;
            }
            "-" => {
                *current_dir = history.clone();
            }
            _ => {
                if &path[0..1] == "/" {
                    println!("cd: no such file or directory: {path}");
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
                        Ok(_) => *current_dir = copy_current_dir,
                        Err(_) => println!("cd: no such file or directory: {path}"),
                    }
                }
            }
        }
    }
}
