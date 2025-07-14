use chrono::Datelike;
use chrono::{DateTime, Local};
use chrono_tz::Tz;
use std::fs;
use std::fs::Metadata;
use std::fs::Permissions;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
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

pub fn format_permissions(permissions: &Permissions, path: &Path) -> String {
    let mode = permissions.mode();
    let owner = (mode & 0o700) >> 6;
    let group = (mode & 0o070) >> 3;
    let others = mode & 0o007;

    let mut perm_str = String::with_capacity(10);

    perm_str.push(if owner & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if owner & 0o2 != 0 { 'w' } else { '-' });
    if mode & 0o4000 != 0 {
        perm_str.push(if owner & 0o1 != 0 { 's' } else { 'S' });
    } else {
        perm_str.push(if owner & 0o1 != 0 { 'x' } else { '-' });
    }

    perm_str.push(if group & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if group & 0o2 != 0 { 'w' } else { '-' });
    if mode & 0o2000 != 0 {
        perm_str.push(if group & 0o1 != 0 { 's' } else { 'S' });
    } else {
        perm_str.push(if group & 0o1 != 0 { 'x' } else { '-' });
    }

    perm_str.push(if others & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if others & 0o2 != 0 { 'w' } else { '-' });
    if mode & 0o1000 != 0 {
        perm_str.push(if others & 0o1 != 0 { 't' } else { 'T' });
    } else {
        perm_str.push(if others & 0o1 != 0 { 'x' } else { '-' });
    }

    // Basic extended attribute check (fallback)
    let attr_len = unsafe {
        libc::listxattr(
            path.to_str().unwrap_or("").as_ptr() as *const _,
            std::ptr::null_mut(),
            0,
        )
    };
    if attr_len > 0 {
        perm_str.push('+');
    }

    perm_str
}

pub fn get_usr(metadata: &Metadata) -> User {
    let uid = metadata.uid();
    let user = match get_user_by_uid(uid) {
        Some(user) => user,
        None => User::new(uid, &uid.to_string(), metadata.gid()),
    };
    user
}
pub fn get_grp(metadata: &Metadata) -> Group {
    let gid = metadata.gid();

    match get_group_by_gid(gid) {
        Some(group) => group,
        None => Group::new(gid, &gid.to_string()), 
    }
}

pub fn get_symlink_target_name(
    symlink_path: &PathBuf,
) -> Result<(Result<Metadata, std::io::Error>, String), String> {
    let meta: Result<Metadata, std::io::Error> = fs::metadata(&symlink_path);

    let target_path = match fs::read_link(&symlink_path) {
        Ok(path) => path,
        Err(err) => {
            return Err(format!(
                "Failed to read symlink '{}': {}",
                symlink_path.display(),
                err
            ));
        }
    };

    Ok((meta, target_path.to_string_lossy().to_string()))
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
