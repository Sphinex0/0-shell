use chrono::{DateTime, Local};
use std::fs::Metadata;
use std::fs::{Permissions, ReadDir};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::*;
use std::{fs, io};
use users::*;

pub fn ls(tab: &[String], current_dir: &PathBuf) -> String {
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
    // let dotdot_metadata = fs::metadata(dotdot_entry);
    // let dot_metadat = fs::metadata(current_dir);

    match entries_result {
        Ok(entries) => match (a_flag, f_flag, l_flag) {
            (true, true, true) => return ls_alF(entries),
            (false, false, true) => return ls_l(entries),
            (false, true, false) => return ls_f(entries),
            (true, false, false) => return myls(entries, true),
            _ => myls(entries, false),
        },
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return format!("Warning: Directory not found: {}", target_dir_str.display());
            }
            io::ErrorKind::PermissionDenied => {
                format!(
                    "Warning: permission denied to read directory: {}",
                    target_dir_str.display()
                )
            }
            _ => {
                format!("Error: Could not read directory: {}", e)
            }
        },
    }
}
fn ls_alF(
    entries: ReadDir,
) -> String {
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        if file_name_str == "." || file_name_str == ".." {
            println!("{}", file_name_str);
        }
    }
    "".to_string()
}
// Classic ls && -a flag
fn myls(entries: ReadDir, showall: bool) -> String {
    let mut items = vec![];

    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_name_os = entry.file_name();
                let name = file_name_os.to_str().unwrap();

                if !name.starts_with('.') && !showall {
                    items.push(name.to_string());
                } else if showall {
                    items.push(name.to_string());
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read directory entry: {}", e)
            }
        }
    }

    items.sort();
    if showall {
        items.insert(0, "..".to_string());
        items.insert(0, ".".to_string());
    }

    items.join(" ")
}

// -F flag //
pub fn ls_f(entries: ReadDir) -> String {
    let mut items = vec![];
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
                    items.push(format!("{}/ ", name));
                } else if file_type.is_symlink() {
                    items.push(format!("{}@ ", name));
                } else if file_type.is_file() {
                    if !name.starts_with('.') {
                        if is_executable(&path) {
                            items.push(format!("{}* ", name));
                        } else {
                            items.push(format!("{} ", name));
                        }
                    }
                }
            }
        }
    }

    items.sort_by(|a, b| a.cmp(b));
    items.join(" ")
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
fn ls_l(entries: ReadDir) -> String {
    let mut total_blocks = 0;
    let mut items = vec![];

    // First pass: gather entries and max widths
    let mut max_user = 0;
    let mut max_group = 0;
    let mut max_size = 0;

    for entry in entries.flatten() {
        let metadata = entry.metadata().unwrap();
        if &entry.file_name().to_str().unwrap()[0..1] == "." {
            continue;
        }

        let user = get_usr(&metadata).unwrap();
        let grp = get_grp(&metadata).unwrap();

        let user_str = user.name().to_str().unwrap().to_string();
        let grp_str = grp.name().to_str().unwrap().to_string();
        let file_size = metadata.size();
        let blocks = metadata.blocks();

        max_user = max_user.max(user_str.len());
        max_group = max_group.max(grp_str.len());
        max_size = max_size.max(file_size.to_string().len());

        items.push((entry, metadata, user_str, grp_str));
        total_blocks += blocks;
    }

    println!(" total {}", total_blocks / 2);
    items.sort_by(|a, b| {
        a.0.file_name()
            .to_string_lossy()
            .cmp(&b.0.file_name().to_string_lossy())
    });

    let mut res = vec![];
    // Second pass: print
    for (entry, metadata, user, grp) in items {
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

        let perms = format_permissions(&permissions);
        let hardlink = metadata.nlink();
        let file_size = metadata.size();
        let last_mdf_time = metadata.modified().unwrap();
        let datetime: DateTime<Local> = last_mdf_time.into();
        let formatted_time = datetime.format("%b %e %H:%M").to_string();

        res.push(format!(
            "{type_char}{perms} {hardlink:2} {:<width_user$} {:<width_grp$} {:<width_size$} {} {}\n",
            user,
            grp,
            file_size,
            formatted_time,
            entry.file_name().to_string_lossy(),
            width_user = max_user,
            width_grp = max_group,
            width_size = max_size
        ));
    }

    " ".to_owned() + &res.join(" ")
}

fn get_usr(metadata: &Metadata) -> Option<User> {
    let uid = metadata.uid();
    let user = get_user_by_uid(uid);
    user
}

fn get_grp(metadata: &Metadata) -> Option<Group> {
    let gid = metadata.gid();
    let grp = get_group_by_gid(gid);
    grp
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
