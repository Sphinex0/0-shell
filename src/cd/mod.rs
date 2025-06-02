use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

use crate::print_error;

// pub fn cd(tab: &[String], current_dir: &mut PathBuf, history: &mut PathBuf, home: &PathBuf) {
//     let tab = tab.join(" ");
//     if tab.len() == 0 {
//         history.push(&current_dir);
//         current_dir.push(home);
//     } else {
//         let mut path = tab;
//         if path != "-" {
//             history.push(&current_dir);
//         }
//         match path.as_str() {
//             "~" => current_dir.push(home),
//             "-" => {
//                 let current_dir_temp = current_dir.clone();
//                 current_dir.push(&history);
//                 history.push(&current_dir_temp);
//             }
//             _ => {
//                 if &path[0..1] == "~" {
//                     path = home.display().to_string() + &path[1..]
//                 }
//                 if &path[0..1] == "/" {
//                     let mut copy_current_dir = current_dir.clone();
//                     copy_current_dir.push(path.clone());
//                     copy_current_dir = copy_current_dir.components().collect::<PathBuf>();
//                     match copy_current_dir.read_dir() {
//                         Ok(_) => *current_dir = copy_current_dir,
//                         Err(err) => print_error(&format!("cd: {err} : {path}")),
//                     }
//                 } else {
//                     let table = path.split("/").collect::<Vec<_>>();
//                     let mut copy_current_dir = current_dir.clone();
//                     for p in table {
//                         match p {
//                             "." => {}
//                             ".." => {
//                                 copy_current_dir.pop();
//                             }
//                             _ => {
//                                 copy_current_dir.push(p);
//                             }
//                         }
//                     }
//                     match copy_current_dir.read_dir() {
//                         Ok(_) => current_dir.push(copy_current_dir),
//                         Err(err) => print_error(&format!("cd: {err} : {path}")),
//                     }
//                 }
//             }
//         }
//     }
// }

pub fn cd(path: &[String], history: &mut PathBuf, current_di: &mut PathBuf, home: &PathBuf) {
    let mut path = path.join(" ");
    if path.len() == 0 {
        path = home.display().to_string();
    }
    let mut change = true;

    // let last_dir = current_di.clone();
    match path.as_str() {
        "-" => {
            if let Err(err) = set_current_dir(history.clone()) {
                change = false;
                print_error(&err.to_string());
            }
        }
        _ => {
            if &path[0..1] == "~" {
                path = home.display().to_string() + &path[1..];
            }
            if let Err(err) = set_current_dir(path) {
                change = false;
                print_error(&err.to_string());
            }
        }
    }

    
    if change {
        history.push(current_di.clone());
        current_di.push(current_dir().unwrap());
    }
}
