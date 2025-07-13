pub fn history(hist: &[String]) -> i32 {
    print!(
        "{}",
        hist.iter()
            .enumerate()
            .map(|(index, command)| format!("{:>5}  {}", index + 1, command))
            .collect::<Vec<_>>()
            .join("")
    );
    0
}
