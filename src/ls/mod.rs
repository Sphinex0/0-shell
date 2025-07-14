pub use helpers::*;
use libc::{major, minor};
use std::fs;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::io;
use std::io::ErrorKind;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use term_size::dimensions;

use crate::print_error;
pub mod helpers;

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
    is_current: bool,
    ticket: bool,
}

impl Ls {
    fn new() -> Self {
        Self {
            files: vec![],
            prev_dir: PathBuf::new(),
            cur_dir: PathBuf::new(),
            a_flag: false,
            f_flag: false,
            l_flag: false,
            files_names: Vec::new(),
            is_current: false,
            ticket: false,
        }
    }

    fn get(&self, path: &str) -> Fileinfo {
        let target_path = if path == "." {
            &self.cur_dir
        } else {
            &self.prev_dir
        };

        let metadata = fs::metadata(target_path).unwrap_or_else(|_| {
            Metadata::from(fs::File::open("/dev/null").unwrap().metadata().unwrap())
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

    fn myls(
        &mut self,
        entries: Vec<DirEntry>,
        file_name: Option<String>,
        is_total: bool,
    ) -> String {
        let mut res = Vec::new();

        // if self.files.len() > 1 {
        //     res.push(format!("{}:\n", file_name.unwrap()));
        // }

        if !self.is_current && file_name.is_some() && self.ticket {
            res.push(format!("{}:\n", file_name.unwrap()));
        }

        let mut max_user = 0;
        let mut max_group = 0;
        let mut max_size = 0;
        let mut max_major = 0;
        let mut max_minor = 0;
        let mut total_blocks = 0;
        let mut max_time_size = 0;
        let mut max_name_size = 0;
        let mut max_link = 0;
        self.files.clear();
        if self.a_flag && self.is_current {
            self.files.push(self.get("."));
            self.files.push(self.get(".."));
        }

        for entry in entries {
            let metadata = entry.metadata().unwrap_or_else(|_| {
                Metadata::from(fs::File::open("/dev/null").unwrap().metadata().unwrap())
            });
            let mut file = Fileinfo::new(metadata.clone());

            let user = helpers::get_usr(&file.metadata);
            let grp = helpers::get_grp(&file.metadata);
            file.user = user.name().to_str().unwrap_or("").to_string();
            file.group = grp.name().to_str().unwrap_or("").to_string();

            let formatted_time = get_time(&file.metadata);
            let rdev = file.metadata.rdev();
            let major_num = major(rdev);
            let minor_num = minor(rdev);
            let size_field = if file.metadata.file_type().is_char_device()
                || file.metadata.file_type().is_block_device()
            {
                max_major = max_major.max(major_num.to_string().len());
                max_minor = max_minor.max(minor_num.to_string().len());
                format!(
                    "{:>width_major$}, {:>width_minor$}",
                    major_num,
                    minor_num,
                    width_major = max_major,
                    width_minor = max_minor
                )
            } else {
                file.metadata.len().to_string()
            };
            max_user = max_user.max(file.user.len());
            max_link = max_link.max(file.metadata.nlink().to_string().len());
            max_group = max_group.max(file.group.len());
            max_size = max_size.max(size_field.len());
            max_time_size = max_time_size.max(formatted_time.len());

            let unsafe_characters = "*?[]$!'\"\\;&|<> ()`~#=";

            let name = entry.file_name().to_string_lossy().into_owned();
            file.name = name.clone();

            for c in name.chars() {
                if unsafe_characters.contains(c) {
                    file.name = "'".to_string() + &file.name + &"'".to_string();
                    break;
                }
            }
            max_name_size = max_name_size.max(file.name.len());

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
            a_tmp.to_ascii_lowercase().cmp(&b_tmp.to_ascii_lowercase())
        });

        let le = self.files.len();
        let term_width = dimensions().map(|(w, _)| w).unwrap_or(80);
        let col_width = max_name_size + 2; // Add padding for spacing
        let total_width = (le * col_width).saturating_sub(2); // Total width without last padding

        // Determine number of rows and columns
        let (num_cols, num_rows) = if total_width <= term_width {
            // Single row if all files fit
            (le, 1)
        } else {
            // Multiple columns based on terminal width
            let num_cols = (term_width / col_width).max(1);
            let num_rows = (le + num_cols - 1) / num_cols;
            (num_cols, num_rows)
        };

        let mut matrix: Vec<Vec<String>> = vec![vec!["".to_string(); num_cols]; num_rows];
        for (i, file) in self.files.iter_mut().enumerate() {
            // Get user and group info
            let user = helpers::get_usr(&file.metadata);
            let grp = helpers::get_grp(&file.metadata);
            file.user = user.name().to_str().unwrap_or("").to_string();
            file.group = grp.name().to_str().unwrap_or("").to_string();
            if !self.a_flag && file.hidden {
                continue;
            }

            if self.l_flag {
                total_blocks += file.metadata.blocks() / 2;

                let permissions = file.metadata.permissions();
                let file_type = file.metadata.file_type();

                let mut color = "\x1b[0m";
                if file.is_exec {
                    color = "\x1b[1;32m";
                }

                let type_char = if file_type.is_dir() {
                    color = "\x1b[1;34m";
                    'd'
                } else if file_type.is_symlink() {
                    color = "\x1b[1;36m";
                    if let Some(en) = &file.entry {
                        if let Ok((meta_data, mut name)) = helpers::get_symlink_target_name(&en) {
                            match meta_data {
                                Ok(meta) => {
                                    let mut color2 = "\x1b[0m";
                                    if meta.is_dir() {
                                        color2 = "\x1b[1;34m";
                                    } else if meta.is_file() && helpers::is_executable(&en) {
                                        color2 = "\x1b[1;32m";
                                    }

                                    if self.f_flag {
                                        if meta.is_dir() {
                                            name.push('/');
                                        } else if meta.is_file() && helpers::is_executable(&en) {
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

                let formatted_time = get_time(&file.metadata);
                let perms = helpers::format_permissions(
                    &permissions,
                    &file.entry.as_ref().unwrap_or(&PathBuf::new()),
                );
                let hardlink = file.metadata.nlink();
                let size_field = if file_type.is_char_device() || file_type.is_block_device() {
                    let rdev = file.metadata.rdev();
                    let major_num = major(rdev);
                    let minor_num = minor(rdev);
                    format!(
                        "{:>width_major$}, {:>width_minor$}",
                        major_num,
                        minor_num,
                        width_major = max_major,
                        width_minor = max_minor
                    )
                } else {
                    file.metadata.len().to_string()
                };

                res.push(format!(
                    "{type_char}{perms} {hardlink:>width_links$} {user:<width_user$} {group:<width_group$} {size:>width_size$} {time:<width_time$} {color}{name}\x1b[0m{newline}",
                    user = file.user,
                    group = file.group,
                    size = size_field,
                    time = formatted_time,
                    name = file.name,
                    width_links = if perms.contains("+") {max_link-1} else {
                    max_link
                    },
                    width_user = max_user,
                    width_group = max_group,
                    width_size = max_size,
                    width_time = max_time_size,
                    newline = if i != le - 1 { "\n" } else { "" },
                ));
                continue;
            } else {
                let row = i % num_rows;
                let col = i / num_rows;

                let mut color = "\x1b[0m";
                let meta = file.metadata.clone();
                if meta.is_dir() {
                    color = "\x1b[1;34m";
                } else if meta.is_symlink() {
                    color = "\x1b[1;36m";
                } else if file.is_exec {
                    color = "\x1b[1;32m";
                }
                let padded_name = if num_rows == 1 {
                    format!("{} ", file.name)
                } else {
                    format!("{:width$}", file.name, width = col_width)
                };
                matrix[row][col] = format!("{}{}\x1b[0m", color, padded_name);
            }
        }

        let mut total_lines = String::new();
        if self.l_flag && is_total {
            total_lines = format!("total {}\n", total_blocks);
        }

        if self.l_flag {
            total_lines + &res.join("")
        } else {
            res.push(
                matrix
                    .into_iter()
                    .filter(|row| row.iter().any(|s| !s.is_empty()))
                    .map(|row| row.join(""))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            res.join("")
        }
    }
}

pub fn ls(tab: &[String], current_dir: &PathBuf) -> i32 {
    let mut ls = Ls::new();
    let mut no_dir = vec![];

    for arg in tab {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => ls.a_flag = true,
                    'F' => ls.f_flag = true,
                    'l' => ls.l_flag = true,
                    _ => {
                        print_error("ls: invalid option -- '{ch}'");
                        return 2;
                    }
                }
            }
        } else {
            let mut path = current_dir.clone();
            path.push(arg.to_string());
            if path.is_file() {
                match dir_entry_from_path(&path) {
                    Ok(entry) => no_dir.push(entry),
                    Err(_) => {}
                }
            } else {
                ls.files_names.push(arg.to_string());
            }
        }
    }

    if ls.files_names.is_empty() && no_dir.is_empty() {
        ls.files_names.push(".".to_string());
        ls.is_current = true;
    }

    let mut output = String::new();

    let le: usize = no_dir.len();

    if !no_dir.is_empty() {
        output.push_str(&ls.myls(no_dir, None, false));
        if !ls.files_names.is_empty() {
            output.push_str("\n\n");
        }
    }

    let mut files = ls.files_names.clone();
    files.sort_by(|a, b| {
        let a_tmp = a
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .collect::<String>();
        let b_tmp = b
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .collect::<String>();
        a_tmp.to_ascii_lowercase().cmp(&b_tmp.to_ascii_lowercase())
    });

    ls.ticket = (files.len() + le) > 1;

    let mut err_status = 0;

    for (i, file_name) in files.iter().enumerate() {
        let mut target_dir_str = current_dir.clone();
        target_dir_str.push(file_name);
        let mut prev_dir = PathBuf::new();
        if target_dir_str.is_dir() {
            prev_dir = target_dir_str.clone();
            prev_dir.push("..");
        }
        ls.cur_dir = target_dir_str.clone();
        ls.prev_dir = prev_dir;

        match fs::read_dir(&target_dir_str) {
            Ok(entries) => {
                let filtered: Vec<_> = entries.filter_map(Result::ok).collect();
                output.push_str(&ls.myls(filtered, Some(file_name.to_string()), true));
                if i != files.len() - 1 {
                    output.push_str("\n");
                }
            }
            Err(err) => {
                err_status = 1;
                let error_message = match err.kind() {
                    ErrorKind::NotFound => format!(
                        "ls: cannot access '{}': No such file or directory",
                        target_dir_str.to_string_lossy()
                    ),
                    ErrorKind::PermissionDenied => format!(
                        "ls: cannot open directory '{}': Permission denied",
                        target_dir_str.to_string_lossy()
                    ),
                    ErrorKind::NotADirectory => {
                        // dbg!("fghfghfgh");
                        // err_status = 0;
                        // let temp_dir = target_dir_str.clone();
                        // let file_name: &str = temp_dir
                        //     .file_name()
                        //     .and_then(|os_str| os_str.to_str())
                        //     .unwrap_or("unknown");
                        // target_dir_str.pop();
                        // match fs::read_dir(&target_dir_str) {
                        //     Ok(entries) => {
                        //         let filtered: Vec<_> = entries
                        //             .filter_map(Result::ok)
                        //             .filter(|entry| entry.file_name() == file_name)
                        //             .collect();
                        //         output.push_str(&ls.myls(
                        //             filtered,
                        //             Some(file_name.to_string()),
                        //             true,
                        //         ));
                        //         if i != files.len() - 1 {
                        //             output.push('\n');
                        //         }
                        //     }
                        //     Err(_) => {
                        //         err_status = 1;
                        //     }
                        // }
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
    println!("{output}");
    err_status
}

fn dir_entry_from_path(path: &Path) -> io::Result<DirEntry> {
    // Get the file name from the path
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path has no file name"))?
        .to_string_lossy();

    // Get the parent directory (defaults to current directory if none)
    let parent = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    // Read the parent directory
    let entries = fs::read_dir(&parent)?;

    // Find the entry matching the file name
    entries
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_string_lossy() == file_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found in directory"))
}
