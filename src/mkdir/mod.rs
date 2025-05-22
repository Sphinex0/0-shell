use std::fs;
use std::path::Path;

pub fn mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }
    for dname in args {
        let path = Path::new(dname);
        if let Err(err) = fs::create_dir(path) {
            eprintln!("mkdir: cannot create directory '{}': {}", dname, err);
        }
    }
}
