use std::path::PathBuf;

use crate::print_error;

pub fn cd(tab: &[String], current_dir: &mut PathBuf, history: &mut PathBuf, home: &PathBuf) {
    if tab.len() == 0 {
        history.push(&current_dir);
        current_dir.push(home);
    } else {
        let path = tab[0].as_str();
        if path != "-" {
            history.push(&current_dir);
        }
        match path {
            "~" => current_dir.push(home),
            "-" => {
                current_dir.push(&history);
            }
            _ => {
                if &path[0..1] == "/" {
                    let mut copy_current_dir = current_dir.clone();
                    copy_current_dir.push(path);
                    copy_current_dir = copy_current_dir.components().collect::<PathBuf>();
                    match copy_current_dir.read_dir() {
                        Ok(_) => *current_dir = copy_current_dir,
                        Err(err) => print_error(&format!("cd: {err} : {path}")),
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
                        Err(err) => print_error(&format!("cd: {err} : {path}")),
                    }
                }
            }
        }
    }
}
