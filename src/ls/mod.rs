use chrono::Datelike;
use chrono::{DateTime, Local};
use std::fs;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::fs::Permissions;
use std::io::ErrorKind;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use users::*;

#[derive(Debug)]
struct Fileinfo {
    name: String,
    hidden: bool,
    user: String,
    group: String,
    metadata: Metadata,
    entry: Option<PathBuf>,
    is_exec: bool,
}

impl Fileinfo {
    fn new(metadata: Metadata) -> Self {
        Self {
            name: String::new(),
            hidden: false,
            user: String::new(),
            group: String::new(),
            metadata,
            entry: None,
            is_exec: false,
        }
    }
}

#[derive(Debug)]
struct Ls {
    files: Vec<Fileinfo>,
    cur_dir: PathBuf,
    prev_dir: PathBuf,
    a_flag: bool,
    f_flag: bool,
    l_flag: bool,
    files_names: Vec<String>,
    is_file: bool,
}
impl Ls {
    fn new(prev_dir: &PathBuf, cur_dir: &PathBuf) -> Self {
        Self {
            files: vec![],
            prev_dir: prev_dir.to_path_buf(),
            cur_dir: cur_dir.to_path_buf(),
            a_flag: false,
            f_flag: false,
            l_flag: false,
            files_names: Vec::new(),
            is_file: false,
        }
    }

    fn get(&self, path: &str) -> Fileinfo {
        let target_path = if path == "." {
            &self.cur_dir
        } else {
            &self.prev_dir
        };

        // Try to get metadata
        let metadata = fs::metadata(target_path).unwrap_or_else(|_| {
            Metadata::from(fs::File::open("/dev/null").unwrap().metadata().unwrap()) // dummy fallback
        });

        let mut name = path.to_string();

        if self.f_flag {
            name.push('/');
        }

        Fileinfo {
            name,
            hidden: true,
            user: String::new(),
            group: String::new(),
            metadata,
            entry: None,
            is_exec: false,
        }
    }

    fn myls(&mut self, entries: Vec<DirEntry>) -> String {
        let mut max_user = 0;
        let mut max_group = 0;
        let mut max_size = 0;
        let mut total_blocks = 0;
        let mut max_time_size = 0;

        self.files.clear();
        if self.a_flag && !self.is_file {
            self.files.push(self.get("."));
            self.files.push(self.get(".."));
        }
        let name = iana_time_zone::get_timezone().unwrap();
        let tz = name.parse::<chrono_tz::Tz>().unwrap();

        for entry in entries {
            let metadata = entry.metadata().unwrap();

            let mut file = Fileinfo::new(metadata.clone());

            // Get user and group info
            let user = get_usr(&metadata).unwrap();
            let grp = get_grp(&metadata);
            file.user = user.name().to_str().unwrap().to_string();
            file.group = grp.name().to_str().unwrap().to_string();
            let last_mod_time = metadata.modified().unwrap();
            let datetime: DateTime<Local> = last_mod_time.into();
            let datetime = datetime.with_timezone(&tz);
            let formatted_time = datetime.format("%b %e %H:%M").to_string();

            max_user = max_user.max(file.user.len());
            max_group = max_group.max(file.group.len());
            max_size = max_size.max(file.metadata.len().to_string().len());
            max_time_size = max_time_size.max(formatted_time.len());

            let name = entry.file_name().to_string_lossy().into_owned();
            file.name = name.clone();
            file.entry = Some(entry.path().clone());

            if name.starts_with('.') {
                file.hidden = true;
            }

            let path = entry.path();
            file.is_exec = is_executable(&path);

            if self.f_flag {
                let file_type = match entry.file_type() {
                    Ok(ft) => ft,
                    Err(err) => {
                        eprintln!("Could not get file type: {}", err);
                        continue;
                    }
                };
                if file_type.is_dir() {
                    file.name.push('/');
                } else if entry.path().is_symlink() && !self.l_flag {
                    file.name.push('@');
                } else if file_type.is_file() && file.is_exec {
                    file.name.push('*');
                }
            }

            if !self.a_flag && file.hidden {
                continue;
            }

            self.files.push(file);
        }

        self.files.sort_by(|a, b| {
            let a_tmp = a
                .name
                .chars()
                .filter(|ch| ch.is_alphanumeric())
                .collect::<String>();
            let b_tmp = b
                .name
                .chars()
                .filter(|ch| ch.is_alphanumeric())
                .collect::<String>();
            a_tmp
                .to_ascii_lowercase()
                .as_bytes()
                .cmp(&b_tmp.to_ascii_lowercase().as_bytes())
        });

        let mut res = Vec::new();
        let le = self.files.len();

        for (i, file) in self.files.iter_mut().enumerate() {
            // Skip hidden files if -a is not set
            if !self.a_flag && file.hidden {
                continue;
            }

            if self.l_flag {
                // Track total blocks
                total_blocks += file.metadata.blocks();
            }

            if self.l_flag {
                let permissions = file.metadata.permissions();
                let file_type = file.metadata.file_type();

                let mut color = "\x1b[0m";
                if file.is_exec {
                    color = "\x1b[1;32m";
                }

                // Determine file type char like ls does
                let type_char = if file_type.is_dir() {
                    color = "\x1b[1;34m";
                    'd'
                } else if file_type.is_symlink() {
                    color = "\x1b[1;36m";
                    if let Some(en) = &file.entry {
                        if let Ok((meta_data, mut name)) = get_symlink_target_name(&en) {
                            match meta_data {
                                Ok(meta) => {
                                    let mut color2 = "\x1b[0m";
                                    if meta.is_dir() {
                                        color2 = "\x1b[1;34m";
                                    } else if meta.is_file() && is_executable(&en) {
                                        color2 = "\x1b[1;32m";
                                    }

                                    if self.f_flag {
                                        // let path = target_file.path();
                                        if meta.is_dir() {
                                            color2 = "\x1b[1;34m";
                                            name.push('/');
                                        } else if meta.is_file() && is_executable(&en) {
                                            name.push('*');
                                        }
                                    }
                                    file.name =
                                        format!("{}\x1b[0m -> {color2}{}\x1b[0m", file.name, name);
                                }
                                Err(_) => {
                                    file.name = format!(
                                        "\x1b[1;31m{}\x1b[0m -> \x1b[1;31m{}\x1b[0m",
                                        file.name, name
                                    );
                                }
                            }
                        }
                    }

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

                let last_mod_time = file.metadata.modified().unwrap();
                let datetime: DateTime<Local> = last_mod_time.into();
                let datetime = datetime.with_timezone(&tz);

                let mut formatted_time = datetime.format("%b %e %H:%M").to_string();

                let current_year = Local::now().year();
                let its_year = datetime.year();
                if current_year != its_year {
                    formatted_time = datetime.format("%b %e  %Y").to_string();
                }

                let perms = format_permissions(&permissions);
                let hardlink = file.metadata.nlink();
                let file_size = file.metadata.len();

                res.push(format!(
                    "{type_char}{perms} {hardlink:2} {user:<width_user$} {group:>width_grp$} {size:>width_size$} {time:>width_time$}  {color}{name}\x1b[0m{newline}",
                    user = file.user,
                    group = file.group,
                    size = file_size,
                    time = formatted_time,
                    name = file.name,
                    width_user = max_user,
                    width_grp = max_group,
                    width_size = max_size,
                    width_time = max_time_size,
                    newline = if i != le - 1 { "\n" } else { "" },
                ));
                continue;
            } else {
                let mut color = "\x1b[0m";
                let meta = file.metadata.clone();
                if meta.is_dir() {
                    color = "\x1b[1;34m";
                } else if meta.is_symlink() {
                    color = "\x1b[1;36m";
                } else if file.is_exec {
                    color = "\x1b[1;32m";
                }
                res.push(format!("{}{}\x1b[0m", color, file.name));
            }
        }

        let mut total_lines = String::new();
        if self.l_flag && !self.is_file {
            total_lines = format!(" total {}\n ", (total_blocks + 1) / 2);
        }
        total_lines + &res.join(" ")
    }
}

pub fn ls(tab: &[String], current_dir: &PathBuf) -> String {
    let target_dir_str = current_dir.clone();
    let mut prev_dir = current_dir.clone();
    prev_dir.push("..");

    let mut ls = Ls::new(&prev_dir, current_dir);

    for arg in tab {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => ls.a_flag = true,
                    'F' => ls.f_flag = true,
                    'l' => ls.l_flag = true,
                    _ => {}
                }
            }
        } else {
            if arg.trim().len() != 0 {
                ls.files_names.push(arg.to_string());
            }
        }
    }

    if ls.files_names.is_empty() {
        ls.files_names.push(".".to_string());
    }

    let mut output = String::new();

    let files = ls.files_names.clone();
    for (i, file_name) in files.iter().enumerate() {
        let mut dir = target_dir_str.clone();
        dir.push(file_name);
        if files.len() > 1 {
            output.push_str(&format!("{}:\n", file_name));
        }
        match fs::read_dir(&dir) {
            Ok(entries) => {
                let filtered: Vec<_> = entries.filter_map(Result::ok).collect();
                output.push_str(&ls.myls(filtered));
                if i != files.len() - 1 {
                    output.push_str("\n");
                }
            }
            Err(err) => {
                let error_message = match err.kind() {
                    ErrorKind::NotFound => format!(
                        "ls: cannot access '{}': No such file or directory",
                        dir.to_string_lossy()
                    ),
                    ErrorKind::PermissionDenied => format!(
                        "ls: cannot open directory '{}': Permission denied",
                        dir.to_string_lossy()
                    ),
                    ErrorKind::NotADirectory => {
                        let temp_dir = dir.clone();
                        let file_name = temp_dir.file_name().unwrap().to_str().unwrap();
                        dir.pop();
                        match fs::read_dir(&dir) {
                            Ok(entries) => {
                                let filtered: Vec<_> = entries
                                    .filter_map(Result::ok)
                                    .filter(|entry| entry.file_name() == file_name)
                                    .collect();
                                ls.is_file = true;
                                output.push_str(&ls.myls(filtered));

                                if i != files.len() - 1 {
                                    output.push('\n');
                                }
                            }
                            Err(_) => {}
                        }
                        format!("")
                    }
                    _ => format!("ls: cannot access '{}': {}", file_name, err),
                };
                output.push_str(&error_message);
            }
        }
        if files.len() > 1 && i != files.len() - 1 {
            output.push('\n');
        }
    }
    output
}

// helpers
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

fn get_grp(metadata: &Metadata) -> Group {
    let gid = metadata.gid();

    match get_group_by_gid(gid) {
        Some(group) => group,
        None => get_group_by_gid(0).unwrap_or(Group::new(gid, "root")),
    }
}

fn get_symlink_target_name<P: AsRef<Path>>(
    symlink_path: P,
) -> Result<(Result<Metadata, std::io::Error>, String), String> {
    // Read the target path of the symlink
    let meta: Result<Metadata, std::io::Error> = fs::metadata(&symlink_path);

    let target_path = match fs::read_link(&symlink_path) {
        Ok(path) => path,
        Err(err) => {
            return Err(format!(
                "Failed to read symlink '{}': {}",
                symlink_path.as_ref().display(),
                err
            ));
        }
    };

    // Get the file name from the target path
    let target_name = match target_path.file_name() {
        Some(name) => name,
        None => {
            return Err(format!(
                "Symlink '{}' points to an invalid path: {}",
                symlink_path.as_ref().display(),
                target_path.display()
            ));
        }
    };

    // Convert OsStr to String
    let name = target_name.to_str().map(String::from).unwrap();

    Ok((meta, name))
}
