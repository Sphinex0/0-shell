pub fn echo(_args: &[String], entry: &String) {
    parse_entry(entry);
}

fn parse_entry(entry: &String) {
    let le: usize = entry.split_whitespace().nth(0).unwrap().len();
    let entry = entry[le + 1..].to_string();
    let mut open_quote = false;
    let mut result: String = String::new();
    let mut chars = entry.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => match chars.next() {
                Some(ch2) => match (ch2, open_quote) {
                    ('n', true) => result.push('\n'),
                    ('r', true) => result.push('\r'),
                    ('t', true) => result.push('\t'),
                    ('a', true) => result.push(7 as char),
                    ('b', true) => result.push(8 as char),
                    ('\\', false) => match chars.next() {
                        Some(ch3) => match ch3 {
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'a' => result.push(7 as char),
                            'b' => result.push(8 as char),
                            _ => {
                                result.push_str(&format!("\\{ch3}"));
                            }
                        },
                        None => result.push('\\'),
                    },
                    a => {
                        result.push(a.0);
                    }
                },
                None => {}
            },
            '"' => {
                open_quote = !open_quote;
            }
            _ => result.push(ch),
        }
    }

    let mut chs = result.chars().peekable();
    let mut res = String::new();
    while let Some(ch) = chs.next() {
        match ch {
            '\\' => match chs.peek() {
                Some('\\') => {
                    res.push('\\');
                    chs.next();
                }
                _ => {}
            },
            _ => res.push(ch),
        }
    }
    print!("{:#?}",res);
    print!("{res}");
}
