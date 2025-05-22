use std::path::PathBuf;

pub fn cd(tab: &[&str], current_dir: &mut PathBuf) {
    if tab.len() == 0 {
        current_dir.clear();
    }
    match tab[0] {
        "../" | ".." => {
            current_dir.pop();
        }
        "." => {}
        mut path => {
            path = match path.strip_prefix("./") {
                Some(p) => p,
                None => path
            } ;
            current_dir.push(path);
            match current_dir.read_dir() {
                Ok(_) => {}
                Err(_) => {
                    current_dir.pop();
                    println!("No such directory")
                }
            }
        }
    };
}
