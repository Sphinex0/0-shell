use std::fs;
use std::path::Path;

pub fn cp(args: &[String]) {
    if args.len() != 2 {
        eprintln!("cp: wrong number of arguments");
        return;
    }
    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);
    if !src.exists() {
        eprintln!("cp: cannot stat '{}': No such file or directory", src.display());
        return;
    }
    if src.is_dir() {
        eprintln!("cp: -r not specified; omitting directory '{}'", src.display());
        return;
    }
    let final_dst = if dst.is_dir() {
        dst.join(src.file_name().unwrap_or_default())
    } else {
        dst.to_path_buf()
    };
    if let Err(err) = fs::copy(src, &final_dst) {
        eprintln!("cp: cannot copy '{}': {}", src.display(), err);
    }
}
