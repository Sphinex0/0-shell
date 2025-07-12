pub fn history(hist: &[String]) -> String {
    hist.iter()
        .enumerate()
        .map(|(index, command)| format!("{} {command}", index + 1))
        .collect::<Vec<_>>()
        .join("")
}
