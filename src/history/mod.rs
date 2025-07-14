pub fn history(hist: &[String]) -> i32 {
    let le = hist.len().to_string().len();
    print!(
        "{}",
        hist.iter()
            .enumerate()
            .map(|(index, command)| format!("{:>le$}  {}", index + 1, command))
            .collect::<Vec<_>>()
            .join("")
    );
    0
}
