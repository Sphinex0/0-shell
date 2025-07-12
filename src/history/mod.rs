pub fn history(hist: &[String]) -> String {
    hist.iter()
        .enumerate()
        .map(|(index, command)| format!("{:>5}  {}", index + 1, command))
        .collect::<Vec<_>>()
        .join("")
}
