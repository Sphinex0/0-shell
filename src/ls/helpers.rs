use chrono::Datelike;
use chrono::{DateTime, Local};
use chrono_tz::Tz;
use std::fs;
use std::fs::Metadata;
use std::fs::Permissions;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;
use users::*;

// helpers
pub fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();
        mode & 0o111 != 0
    } else {
        false
    }
}

pub fn format_permissions(permissions: &Permissions) -> String {
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

pub fn get_usr(metadata: &Metadata) -> User {
    let uid = metadata.uid();
    let user =  match get_user_by_uid(uid) {
        Some(group) => group,
        None => get_user_by_uid(0).unwrap_or(User::new(uid, "root", metadata.gid())),
    };
    user
}

pub fn get_grp(metadata: &Metadata) -> Group {
    let gid = metadata.gid();

    match get_group_by_gid(gid) {
        Some(group) => group,
        None => get_group_by_gid(0).unwrap_or(Group::new(gid, "root")),
    }
}

pub fn get_symlink_target_name<P: AsRef<Path>>(
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
    let name = target_name
        .to_str()
        .map(String::from)
        .unwrap_or("".to_string());

    Ok((meta, name))
}

pub fn get_time(metadata: &Metadata) -> String {
    let name = iana_time_zone::get_timezone().unwrap_or("UTC".to_string());
    let tz = name.parse::<chrono_tz::Tz>().unwrap_or(Tz::UTC);
    let last_mod_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = last_mod_time.into();
    let datetime = datetime.with_timezone(&tz);

    let mut formatted_time = datetime.format("%b %e %H:%M").to_string();
    let current_year = Local::now().year();
    let its_year = datetime.year();
    if current_year != its_year {
        formatted_time = datetime.format("%b %e  %Y").to_string();
    };
    formatted_time
}
