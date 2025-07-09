pub fn echo(args: &[String], _entry: &String)-> String {
    // println!("echo args:{args:?}");
    let entry = args.join("");
    let (res, newline) = parse_entry(&entry);
    if newline {
        format!("{}", res)
    } else {
        format!("{}", res)
    }
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

// fn parse_entry(entry: &String) {
//     let le: usize = entry.split_whitespace().nth(0).unwrap().len();
//     let entry: String = entry[le + 1..].to_string();
//     // let mut open_quote = false;
//     let mut result: String = String::new();
//     let mut chars = entry.chars().peekable();
//     while let Some(ch) = chars.next() {
//         match ch {
//             '\\' => match chars.next() {
//                 Some(ch2) => match (ch2) {
//                     ('n', true) => result.push('\n'),
//                     ('r', true) => result.push('\r'),
//                     ('t', true) => result.push('\t'),
//                     ('a', true) => result.push(7 as char),
//                     ('b', true) => result.push(8 as char),
//                     ('\\', false) => {
//                         /*
//                             match chars.next() {
//                             Some(ch3) => match ch3 {
//                                 'n' => result.push('\n'),
//                                 'r' => result.push('\r'),
//                                 't' => result.push('\t'),
//                                 'a' => result.push(7 as char),
//                                 'b' => result.push(8 as char),
//                                 _ => {
//                                     result.push_str(&format!("\\{ch3}"));
//                                 }
//                             },
//                             None => result.push('\\'),
//                         } */
//                        result.push('\\');
//                     }
//                     a => {
//                         result.push(a.0);
//                     }
//                 },
//                 None => {}
//             },
//             '"' => {
//                 open_quote = !open_quote;
//             }
//             _ => result.push(ch),
//         }
//     }

//     // let mut chs = result.chars().peekable();
//     // let mut res = String::new();
//     // while let Some(ch) = chs.next() {
//     //     match ch {
//     //         '\\' => match chs.peek() {
//     //             Some('\\') => {
//     //                 res.push('\\');
//     //                 chs.next();
//     //             }
//     //             _ => {}
//     //         },
//     //         _ => res.push(ch),
//     //     }
//     // }
//     // print!("{:#?}", res);
//     print!("{res}");
// }
