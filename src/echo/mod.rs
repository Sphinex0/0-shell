pub fn echo(args: &[String]) -> i32 {
    let entry = args.join(" ");

    let (content, newline) = parse_entry(&entry);
    if newline{
        println!("{content}");
    }else{
        print!("{content}")
    }
    0
}

fn parse_entry(entry: &str) -> (String, bool) {
    let mut result = String::new();
    let mut chars = entry.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'a' => result.push('\x07'),
                        'b' => result.push('\x08'),
                        'c' => return (result, false),
                        'e' => result.push('\x1B'),
                        'f' => result.push('\x0C'),
                        'v' => result.push('\x0B'),
                        '\\' => result.push('\\'),
                        '0' => match chars.peek() {
                            //1
                            Some(&ch) => {
                                let mut octal = String::new();
                                if ch.is_digit(8) {
                                    octal.push(ch);
                                    chars.next();
                                    //2
                                    match chars.peek() {
                                        Some(&ch2) => {
                                            if ch2.is_digit(8) {
                                                octal.push(ch2);
                                                chars.next();
                                                //3
                                                match chars.peek() {
                                                    Some(&ch3) => {
                                                        if ch3.is_digit(8) {
                                                            octal.push(ch3);
                                                            chars.next();
                                                        }
                                                    }
                                                    None => {}
                                                };
                                            }
                                        }
                                        None => {}
                                    };
                                }
                                match u8::from_str_radix(&octal, 8) {
                                    Ok(val) => result.push(val as char),
                                    _ => {}
                                }
                            }
                            None => {}
                        },
                        _ => {
                            result.push(ch);
                            result.push(next_ch);
                        }
                    }
                } else {
                    result.push('\\');
                }
            }
            _ => result.push(ch),
        }
    }

    (result, true)
}

//   \b      A backspace character is output.

//             \c      Subsequent output is suppressed.  This is normally used at the end of the last argument to suppress the trailing newline that echo would otherwise output.

//             \e      Outputs an escape character (ESC).

//             \f      Output a form feed.

//             \n      Output a newline character.

//             \r      Output a carriage return.

//             \t      Output a (horizontal) tab character.

//             \v      Output a vertical tab.

//             \0digits
//                     Output the character whose value is given by zero to three octal digits.  If there are zero digits, a nul character is output.

//             \\      Output a backslash.

//             All other backslash sequences elicit undefined behaviour.
