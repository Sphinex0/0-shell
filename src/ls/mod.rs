use std::fs::{Permissions, ReadDir};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::FileTypeExt;
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
            (false, false, true) => ls_l(entries),
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

// Classic ls && -a flag
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

// -F flag //
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

// ls -l
fn ls_l(entries: ReadDir) {
    for entry in entries {
        match entry {
            Ok(entry) => {
                let metadata = entry.metadata().expect("Could not read entry");
                let permissions = metadata.permissions();
                let file_type = metadata.file_type();
                let type_char = if file_type.is_dir() {
                    'd'
                } else if file_type.is_symlink() {
                    'l'
                } else if file_type.is_socket() {
                    's'
                } else if file_type.is_fifo() {
                    'p'
                } else if file_type.is_char_device() {
                    'c'
                } else if file_type.is_block_device() {
                    'b'
                } else if file_type.is_file() {
                    '-'
                } else {
                    '?'
                };

                let permissions = format_permissions(&permissions);
                if &entry.file_name().to_str().unwrap()[0..1] != "." {
                    println!(
                        "{type_char}{} {}",
                        permissions,
                        entry.file_name().to_string_lossy()
                    );
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read directory entry: {}", e)
            }
        }
    }
    println!();
}

fn format_permissions(permissions: &Permissions) -> String {
    let mode = permissions.mode();
    let owner = mode >> 6;
    let group = mode >> 3;
    let others = mode;

    let mut perm_str = String::with_capacity(9);
    perm_str.push(if owner & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if owner & 0o2 != 0 { 'w' } else { '-' });
    perm_str.push(if owner & 0o1 != 0 { 'x' } else { '-' });

    perm_str.push(if group & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if group & 0o2 != 0 { 'w' } else { '-' });
    perm_str.push(if group & 0o1 != 0 { 'x' } else { '-' });

    perm_str.push(if others & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if others & 0o2 != 0 { 'w' } else { '-' });
    perm_str.push(if others & 0o1 != 0 { 'x' } else { '-' });

    perm_str
}
