use std::path::PathBuf;

pub fn cd(tab: &[&str], current_dir: &mut PathBuf) {
    match tab[0] {
        "../" => {
            current_dir.pop();
        }
        path => {
            current_dir.push(path);
            match current_dir.read_dir() {
                Ok(_) => {},
                Err(_) => {
                    current_dir.pop();
                    println!("No such directory")
                },
            }
        }
    };
}
