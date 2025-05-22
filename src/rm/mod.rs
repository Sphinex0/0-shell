use std::fs;
use std::path::Path;

pub fn rm(args: &[&str]) {
    let mut r = false;
    let mut names = vec![];
    for a in args {
        if a == &"-r" {
            r = true;
        } else {
            names.push(a);
        }
    }
    if names.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    for name in names {
        let p = Path::new(name);
        let res = if p.is_dir() && r {
            fs::remove_dir_all(p)
        } else if p.is_dir() {
            eprintln!("rm: {}: is a directory", name);
            return;
        } else {
            fs::remove_file(p)
        };
        if let Err(e) = res {
            eprintln!("rm: {}: {}", name, e);
        }        
    }
}
