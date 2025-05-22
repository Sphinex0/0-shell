pub fn cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: missing file name");
        return;
    } else {
        for filename in args {
            match std::fs::read_to_string(filename) {
                Ok(contents) => print!("{}", contents),
                Err(err) => eprintln!("cat: {}: {}", filename, err),
            }
        }
    }
}