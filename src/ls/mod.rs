use std::path::*;
use std::{fs, io};

pub fn ls(tab: &[String], current_dir: &PathBuf) {
    let mut target_dir_str = current_dir.clone();
    let mut a_flag = false;
    let mut f_flag = false;
    let mut l_flag = false;

    for flag in tab {
        let target = flag.trim_start_matches('-');
        match target {
            "a" => a_flag = true,
            "f" => f_flag = true,
            "l" => l_flag = true,
            _ => {
                target_dir_str.push(target);
            }
        }
    }
    // read directory content
    let entries_result = fs::read_dir(&target_dir_str);

    match entries_result {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name_os = entry.file_name();
                        if &file_name_os.to_str().unwrap()[0..1] != "."  {
                            print!("{} ", file_name_os.to_str().unwrap());
                            continue;
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not read directory entry: {}", e)
                    }
                }
            }
            println!();
        }
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
