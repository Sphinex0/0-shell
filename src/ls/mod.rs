use std::fs::ReadDir;
use std::os::unix::fs::PermissionsExt;
use std::path::*;
use std::{fs, io};

pub fn ls(tab: &[&str], current_dir: &PathBuf) {
    let mut target_dir_str = current_dir.clone();
    let mut a_flag = false;
    let mut f_flag = false;
    let mut l_flag = false;

    for flag in tab {
        let target = flag.trim_start_matches('-');
        match target {
            "a" => a_flag = true,
            "F" => f_flag = true,
            "l" => l_flag = true,
            _ => {
                target_dir_str.push(target);
            }
        }
    }
    // read directory content
    let entries_result = fs::read_dir(&target_dir_str);

    match entries_result {
        Ok(entries) => match (a_flag, f_flag, l_flag) {
            (false, true, false) => {
                ls_f(entries);
            }
            (true, false, false) => {
                print!(". ");
                print!(".. ");
                myls(entries, true);
            }
            _ => {
                myls(entries, false);
            }
        },
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("Warning: Directory not found: {}", target_dir_str.display());
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "Warning: permission denied to read directory: {}",
                    target_dir_str.display()
                );
            }
            _ => {
                eprintln!("Error: Could not read directory: {}", e);
            }
        },
    }
}

fn myls(entries: ReadDir, dr: bool) {
    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_name_os = entry.file_name();
                if &file_name_os.to_str().unwrap()[0..1] != "." && !dr {
                    print!("{} ", file_name_os.to_str().unwrap());
                    continue;
                } else if dr {
                    print!("{} ", file_name_os.to_str().unwrap());
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read directory entry: {}", e)
            }
        }
    }
    println!();
}


pub fn ls_f(entries: ReadDir) {
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    eprintln!("Could not get file type: {}", e);
                    continue;
                }
            };

            if let Some(name) = entry.file_name().to_str() {
                if file_type.is_dir() {
                    print!("{}/ ", name);
                } else if file_type.is_symlink() {
                    print!("{}@ ", name);
                } else if file_type.is_file() {
                    if !name.starts_with('.') {
                        if is_executable(&path) {
                            print!("{}* ", name);
                        } else {
                            print!("{} ", name);
                        }
                    }
                }
            }
        }
    }
    println!();
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();
        mode & 0o111 != 0
    } else {
        false
    }
}