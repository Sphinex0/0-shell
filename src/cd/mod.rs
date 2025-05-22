use std::path::PathBuf;

pub fn cd(tab: &[&str], current_dir: &mut PathBuf) {
    if tab.len() == 0 {
        current_dir.clear();
        current_dir.push("/");
        return;
    }
    let path = tab[0];
    let table = path.split("/").collect::<Vec<_>>();
    let mut copy_current_dir = current_dir.clone();
    for p in table {
        match p {
            "." => {}
            ".." => {
                copy_current_dir.pop();
            }
            _ => {
                copy_current_dir.push(p);
            }
        }
    }
    match copy_current_dir.read_dir() {
        Ok(_) => *current_dir = copy_current_dir,
        Err(err) => println!("cd: no such file or directory: {path}"),
    }
}
