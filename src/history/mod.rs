pub fn history(hist: &Vec<&str>) {
    for command in hist {
        println!("{}", command);
    }
}
