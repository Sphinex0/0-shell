pub use helpers::*;
use libc::{major, minor};
use std::fs;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::io::ErrorKind;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
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
    is_file: bool,
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
            is_file: false,
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

    fn myls(&mut self, entries: Vec<DirEntry>) -> String {
        let mut max_user = 0;
        let mut max_group = 0;
        let mut max_size = 0;
        let mut total_blocks = 0;
        let mut max_time_size = 0;
        let mut max_name_size = 0;
        let mut max_mijor = 0;
        let mut max_minor = 0;
        self.files.clear();
        if self.a_flag && !self.is_file {
            self.files.push(self.get("."));
            self.files.push(self.get(".."));
        }

        for entry in entries {
            let metadata = entry.metadata().unwrap_or_else(|_| {
                Metadata::from(fs::File::open("/dev/null").unwrap().metadata().unwrap())
            });
            let mut file = Fileinfo::new(metadata.clone());

            let formatted_time = get_time(&file.metadata);
            let rdev = file.metadata.rdev();
            let major_num = major(rdev);
            let minor_num = minor(rdev);
            max_mijor = max_mijor.max(major_num.to_string().len());
            max_minor = max_minor.max(minor_num.to_string().len());
            max_user = max_user.max(file.user.len());
            max_group = max_group.max(file.group.len());
            max_size = max_size.max(file.metadata.len().to_string().len());
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
            a_tmp
                .to_ascii_lowercase()
                .as_bytes()
                .cmp(&b_tmp.to_ascii_lowercase().as_bytes())
        });

        let mut res = Vec::new();
        let le = self.files.len();
        let term_width = dimensions().map(|(w, _)| w).unwrap_or(80);
        let col_width = max_name_size + 2; // Add padding for spacing
        let total_width = le * col_width - 2; // Total width without last padding

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
        let mut add_5 = 0;
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
                total_blocks += file.metadata.blocks();

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
                                            color2 = "\x1b[1;34m";
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
                    add_5 = 5;
                    'c'
                } else if file_type.is_block_device() {
                    'b'
                } else if file_type.is_file() {
                    '-'
                } else {
                    '?'
                };

                let formatted_time = get_time(&file.metadata);
                let perms = helpers::format_permissions(&permissions);
                let hardlink = file.metadata.nlink();
                let file_size = file.metadata.len();
                let size_field = if file_type.is_char_device() || file_type.is_block_device() {
                    let rdev = file.metadata.rdev();
                    let major_num = major(rdev);
                    let minor_num = minor(rdev);
                    format!(
                        "{:<major_width$} {:>minor_width$}",
                        format!("{major_num},"),
                        minor_num,
                        major_width = max_mijor,
                        minor_width = max_minor,
                    )
                } else {
                    file_size.to_string()
                };

                res.push(format!(
                "{type_char}{perms} {hardlink:2} {user:<width_user$} {group:<width_grp$} {size:<width_size$} {time:>width_time$}  {color}{name}\x1b[0m{newline}",
                user = file.user,
                group = file.group,
                size = size_field,
                time = formatted_time,
                name = file.name,
                width_user = max_user,
                width_grp = max_group,
                width_size = if max_size+add_5 < max_minor + max_mijor {max_minor + max_mijor} else {max_size+add_5},
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
        if self.l_flag && !self.is_file {
            total_lines = format!("total {}\n", (total_blocks + 1) / 2);
        }

        if self.l_flag {
            total_lines + &res.join("")
        } else {
            // Convert matrix to string, joining non-empty rows
            matrix
                .into_iter()
                .filter(|row| row.iter().any(|s| !s.is_empty()))
                .map(|row| row.join(""))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

pub fn ls(tab: &[String], current_dir: &PathBuf) -> i32 {
    let mut ls = Ls::new();

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
            ls.files_names.push(arg.to_string());
        }
    }

    if ls.files_names.is_empty() {
        ls.files_names.push(".".to_string());
    }

    let mut output = String::new();

    let files = ls.files_names.clone();
    let mut err_status = 0;

    for (i, file_name) in files.iter().enumerate() {
        let mut target_dir_str = current_dir.clone();
        target_dir_str.push(file_name);
        let mut prev_dir = target_dir_str.clone();
        prev_dir.push("..");
        let mut dir = target_dir_str.clone();

        ls.cur_dir = target_dir_str.clone();
        ls.prev_dir = prev_dir.clone();
        
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
                err_status = 1;
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
                        err_status = 0;
                        let temp_dir = dir.clone();
                        let file_name: &str = temp_dir
                            .file_name()
                            .and_then(|os_str| os_str.to_str())
                            .unwrap_or("unknown");
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
                                // dbg!( &output);
                            }
                            Err(_) => {
                                err_status = 1;
                            }
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
    println!("{output}");
    err_status
}