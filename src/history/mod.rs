pub fn history(hist: &[String], num: Vec<String>) {
    match num.len() {
        0 => {
            print!("{}", hist.join(""));
        }
        1 => {
            let number: i32 = match num[0].parse::<i32>() {
                Ok(n) => n,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return;
                }
            };
            let index = if number < 0 {
                let adjusted = hist.len() as i32 + number;
                if adjusted < 0 {
                    eprintln!("Error: Index out of bounds");
                    return;
                }
                adjusted as usize
            } else {
                number as usize
            };
            if index >= hist.len() {
                eprintln!("Error: Index out of bounds");
                return;
            }
            print!("{}", hist[index..].join(""));
        }
        2 => {
            let number1 = match num[0].parse::<i32>() {
                Ok(n) => n,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return;
                }
            };
            let number2 = match num[1].parse::<i32>() {
                Ok(n) => n,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return;
                }
            };
            let start = if number1 < 0 {
                let adjusted = hist.len() as i32 + number1;
                if adjusted < 0 {
                    eprintln!("Error: Start index out of bounds");
                    return;
                }
                adjusted as usize
            } else {
                number1.saturating_sub(1) as usize
            };
            let end = if number2 < 0 {
                let adjusted = hist.len() as i32 + number2;
                if adjusted < 0 {
                    eprintln!("Error: End index out of bounds");
                    return;
                }
                adjusted as usize
            } else {
                number2 as usize
            };
            if start >= hist.len() || end > hist.len() || start >= end {
                eprintln!("Error: Invalid range");
                return;
            }
            print!("{}", hist[start..end].join(""));
        }
        _ => {
            eprintln!("Error: Too many arguments");
            return;
        }
    }
}