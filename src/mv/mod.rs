use std::fs;
use std::path::Path;

pub fn mv(raw_args: &[String]) {
    let args: Vec<String> = raw_args.iter()
        .filter(|s| !s.trim().is_empty())
        .cloned()
        .collect();
    if args.is_empty() {
        eprintln!("mv: missing file operand");
        return;
    }
    if args.len() == 1 {
        eprintln!("mv: missing destination file operand after '{}'", args[0]);
        return;
    }
    let last = Path::new(&args[args.len() - 1]);
    let sources = &args[..args.len() - 1];
    if sources.len() > 1 && !last.is_dir() {
        eprintln!("mv: target '{}' is not a directory", last.display());
        return;
    }
    for src_str in sources {
        if src_str.trim().is_empty() {
            continue;
        }

        let src = Path::new(src_str);

        if !src.exists() {
            eprintln!("mv: cannot stat '{}': No such file or directory", src.display());
            continue;
        }

        let dst_path = if last.is_dir() {
            match src.file_name() {
                Some(name) => last.join(name),
                None => {
                    eprintln!("mv: cannot move '{}': invalid file name", src.display());
                    continue;
                }
            }
        } else {
            last.to_path_buf()
        };

        if let Err(e) = fs::rename(src, &dst_path) {
            match fs::copy(src, &dst_path) {
                Ok(_) => {
                    if let Err(e) = fs::remove_file(src) {
                        eprintln!("mv: cannot remove '{}': {}", src.display(), e);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "mv: cannot move '{}' to '{}': {}",
                        src.display(),
                        dst_path.display(),
                        e
                    );
                }
            }
        }
    }
}



//     if args.len() != 2 {
//         eprintln!("mv: wrong number of arguments");
//     } else {
//         let source = Path::new(args[0]);
//         let destination = Path::new(args[1]);
//         if let Err(e) = move_item(source, destination) {
//             eprintln!("mv: {}: {}", source.display(), e);
//         }
//     }
// }
//     let destination = if destination.is_dir() {
//         destination.join(
//             source
//                 .file_name()
//                 .ok_or_else(|| "Invalid file name".to_string())?,
//         )
//     } else {
//         destination.to_path_buf()
//     };

//     fs::rename(source, destination).map_err(|e| e.to_string())
