use crate::print_error;
use std::fs;
use std::path::Path;

pub fn cp(args: &[String])-> i32 {
    if args.len() < 2 {
        print_error("cp: wrong number of arguments");
        return 1;
    }
    let dst = Path::new(&args[args.len() - 1]);
    if args.len() > 2 && !dst.is_dir() {
        print_error(&format!("cp: target '{}' is not a directory", dst.display()));
        return 1;
    }
    for src_str in &args[..args.len() - 1] {
        let src = Path::new(src_str);
        if !src.exists() {
            print_error(&format!("cp: cannot stat '{}': No such file or directory", src.display()));
            continue;
        }
        if src.is_dir() {
            print_error(&format!("cp: -r not specified; omitting directory '{}'", src.display()));
            continue;
        }
        let final_dst = if dst.is_dir() {
            dst.join(src.file_name().unwrap_or_default())
        } else {
            dst.to_path_buf()
        };
        if let Err(err) = fs::copy(src, &final_dst) {
            print_error(&format!("cp: cannot copy '{}': {}", src.display(), err));
        }
    }
    0
}
