use std::path::*;
use std::process;
use std::{fs, io};

pub fn ls(tab: &[&str], current_dir: &PathBuf) {
    let target_dir_str = current_dir;

    //create a path object
   // let target_path = Path::new(target_dir_str);

    // read directory content
    let entries_result = fs::read_dir(target_dir_str);

    match entries_result {
        Ok(entries) => {
            for entry in entries {
                // let res: String
                match entry {
                    Ok(entry) => {
                        let file_name_os = entry.file_name();
                        println!("{}", file_name_os.to_string_lossy())
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not read directory entry: {}", e)
                    }
                }
            }
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
