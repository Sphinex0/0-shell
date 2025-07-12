use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

use crate::print_error;

pub fn cd(tab: &[String], history: &mut PathBuf, current_di: &mut PathBuf, home: &PathBuf) -> i32 {
    let mut path = tab.get(0).unwrap_or(&home.display().to_string()).clone();
    let mut change = true;
    match path.as_str() {
        "-" => {
            if let Err(err) = set_current_dir(history.clone()) {
                change = false;
                print_error(&err.to_string());
            }
        }
        _ if path.len() != 0 => {
            if &path[0..1] == "~" {
                path = home.display().to_string() + &path[1..];
            }
            if let Err(err) = set_current_dir(path) {
                change = false;
                print_error(&err.to_string());
            }
        }
        _ => {}
    }

    if change {
        history.push(current_di.clone());
        current_di.push(current_dir().unwrap());
        0
    } else {
        1
    }
}
