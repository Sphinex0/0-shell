use std::fs;
use std::path::Path;

pub fn mv(args: &[String]) {
    if args.len() != 2 {
        eprintln!("mv: wrong number of arguments");
        return;
    }
    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);
    let dst = if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.join(name),
            None => {
                eprintln!("mv: invalid file name");
                return;
            }
        }
    } else {
        dst.to_path_buf()
    };
    if let Err(e) = fs::rename(src, &dst) {
        let copied = fs::copy(src, &dst);
        if copied.is_ok() {
            let deleted = fs::remove_file(src);
            if deleted.is_err() {
                eprintln!("mv: can't delete {}: {}", src.display(), deleted.unwrap_err());
            }
        } else {
            eprintln!("mv: {}: {}", src.display(), copied.unwrap_err());
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
