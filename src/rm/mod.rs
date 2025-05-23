use std::fs;
use std::path::PathBuf;
pub fn rm(args: &[String], current_dir: &PathBuf) {
    let path_copy = &mut current_dir.clone();
        for arg in args {
            if arg != &"-r" {
                let mut tmp = PathBuf::from(arg);
                if !arg.starts_with("/"){
                    tmp = path_copy.clone();
                    tmp.push(arg);

                }
                match tmp.read_dir(){
                    Ok(_files)=>{
                        fs::remove_dir_all(tmp).unwrap()
                    },
                    Err(_err)=>fs::remove_file(tmp).unwrap()
                }
            }
        }
}


