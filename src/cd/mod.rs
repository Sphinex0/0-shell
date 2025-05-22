use std::path::PathBuf;

pub fn cd(tab: &[&str], current_dir: &mut PathBuf) {
    if tab.len() == 0 {
        current_dir.clear();
        current_dir.push("/");
        return;
    }
    match tab[0] {
        "../" | ".." => {
            current_dir.pop();
        }
        "." => {}
        mut path => {
            path = match path.strip_prefix("./") {
                Some(p) => p,
                None => path,
            };
            let mut copy_current_dir = current_dir.clone();
            copy_current_dir.push(path);
            match copy_current_dir.read_dir() {
                Ok(_) => current_dir.push(path),
                Err(err) => println!("No such directory err {:?}",err),
            }
        }
    };
}
