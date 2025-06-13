use std::fs;
use std::path::Path;

pub fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("cp: wrong number of arguments");
        return;
    }
    let src = Path::new(args[0]);
    let dst = Path::new(args[1]);
    if src.is_dir() {
        eprintln!("cp: '{}' is a directory", src.display());
        return;
    }
    let final_dst = if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.join(name),
            None => {
                eprintln!("cp: invalid file name");
                return;
            }
        }
    } else {
        dst.to_path_buf()
    };
    if let Err(err) = fs::copy(src, final_dst) {
        eprintln!("cp: {}: {}", src.display(), err);
    }
}