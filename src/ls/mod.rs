use chrono::{DateTime, Local};
use std::fs;
use std::fs::Metadata;
use std::fs::*;
use std::fs::{Permissions, ReadDir};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::*;
use users::*;

#[derive(Debug)]
struct Fileinfo {
    name: String,
    trimed_name: String,
    hidden: bool,
    user: String,
    group: String,
    entry: DirEntry,
    metadata: Metadata,
}
impl Fileinfo {
    fn new(entry: DirEntry, metadata: Metadata) -> Self {
        let file_name_os = entry.file_name();
        let name = file_name_os.to_string_lossy().to_string();
        let hidden = name.starts_with('.');

        let trimed_name = if hidden {
            trime_dots(name.clone())
        } else {
            name.clone()
        };

        Self {
            name,
            trimed_name,
            hidden,
            user: String::new(),
            group: String::new(),
            entry,
            metadata,
        }
    }
}

#[derive(Debug)]
struct ls {
    files: Vec<Fileinfo>,
    cur_dir: PathBuf,
    prev_dir: PathBuf,
    a_flag: bool,
    f_flag: bool,
    l_flag: bool,
}
impl ls {
    fn new(prev_dir: &PathBuf, cur_dir: &PathBuf) -> Self {
        Self {
            files: vec![],
            prev_dir: prev_dir.to_path_buf(),
            cur_dir: cur_dir.to_path_buf(),
            a_flag: false,
            f_flag: false,
            l_flag: false,
        }
    }

    fn myls(&mut self, entries: ReadDir) -> String {
        let mut max_user = 0;
        let mut max_group = 0;
        let mut max_size = 0;
        let mut total_blocks = 0;

        self.files.clear();

        for entry in entries.flatten() {
            let metadata = entry.metadata().unwrap();

            let mut file = Fileinfo::new(entry, metadata);

            let name = file.entry.file_name().to_string_lossy().into_owned();
            file.name = name.clone();

            if name.starts_with('.') {
                file.trimed_name = trime_dots(name.clone());
                file.hidden = true;
            }

            if self.f_flag {
                let file_type = match file.entry.file_type() {
                    Ok(ft) => ft,
                    Err(err) => {
                        eprintln!("Could not get file type: {}", err);
                        continue;
                    }
                };
                let path = file.entry.path();
                if file_type.is_dir() {
                    file.name.push('/');
                } else if file_type.is_symlink() {
                    file.name.push('@');
                } else if file_type.is_file() && is_executable(&path) {
                    file.name.push('*');
                }
            }

            // Get user and group info
            let user = get_usr(&file.metadata).unwrap();
            let grp = get_grp(&file.metadata).unwrap();
            file.user = user.name().to_str().unwrap().to_string();
            file.group = grp.name().to_str().unwrap().to_string();

            max_user = max_user.max(file.user.len());
            max_group = max_group.max(file.group.len());
            max_size = max_size.max(file.metadata.len().to_string().len());
            if !self.a_flag && file.hidden {
                continue;
            }

            // Track total blocks
            total_blocks += file.metadata.blocks();

            self.files.push(file);
        }

        self.files
            .sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

        let mut res = Vec::new();

        for file in &self.files {
            // Skip hidden files if -a is not set
            if !self.a_flag && file.hidden {
                continue;
            }

            if self.l_flag {
                let permissions = file.metadata.permissions();
                let file_type = file.metadata.file_type();

                // Determine file type char like ls does
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
                let hardlink = file.metadata.nlink();
                let file_size = file.metadata.len();

                let last_mod_time = file.metadata.modified().unwrap();
                let datetime: DateTime<Local> = last_mod_time.into();
                let formatted_time = datetime.format("%b %e %H:%M").to_string();

                res.push(format!(
                    "{type_char}{perms} {hardlink:2} {:<width_user$} {:<width_grp$} {:>width_size$} {} {}\n",
                    file.user,
                    file.group,
                    file_size,
                    formatted_time,
                    file.name,
                    width_user = max_user,
                    width_grp = max_group,
                    width_size = max_size
                ));
                continue;
            } else {
                res.push(format!("{}", file.name));
            }
        }

        let total_lines = format!("total {}\n", (total_blocks + 1) / 2);
        total_lines + &res.join(" ")
    }

    // gets the current directory and the prev directory meta data
    // .. && .
    // fn prev_cur_dir_metadata(self) -> (Fileinfo, Fileinfo) {
    //     /*get the current dir and the prev file infos*/
    // }
}

pub fn ls(tab: &[String], current_dir: &PathBuf) -> String {
    let mut target_dir_str = current_dir.clone();
    let mut prev_dir = current_dir.clone();
    prev_dir.push("/..");
    let mut ls = ls::new(&prev_dir, current_dir);

    for flag in tab {
        for (i, f) in flag.chars().enumerate() {
            if i == 0 && f == '-' {
                continue;
            }
            match f {
                'a' => ls.a_flag = true,
                'F' => ls.f_flag = true,
                'l' => ls.l_flag = true,
                _ => {
                    target_dir_str.push(flag);
                    break;
                }
            }
        }
    }

    // read directory content
    let entries_result = fs::read_dir(&target_dir_str);
    match entries_result {
        Ok(entries) => return ls.myls(entries),
        Err(_) => {
            return format!(
                "ls: cannot access '{}': No such file or directory",
                &target_dir_str.to_string_lossy()
            );
        }
    }
}

// helpers
fn trime_dots(name: String) -> String {
    for (i, char) in name.chars().enumerate() {
        if char == ' ' {
            continue;
        }
        return name[i..].to_string();
    }
    return "".to_string();
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();
        mode & 0o111 != 0
    } else {
        false
    }
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
