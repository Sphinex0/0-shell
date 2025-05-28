pub mod cat;
pub mod cd;
pub mod cp;
pub mod echo;
pub mod history;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod rm;
pub use cat::*;
pub use cd::*;
pub use cp::*;
pub use echo::*;
pub use history::*;
pub use ls::*;
pub use mkdir::*;
pub use mv::*;
pub use pwd::*;
pub use rm::*;

pub trait CustomSplit {
    fn custom_split(&self) -> (String, bool);
}

impl CustomSplit for String {
    fn custom_split(&self) -> (String, bool) {
        let mut result: String = String::new();
        let mut open_double_quote = false;
        let mut open_single_quote = false;
        let mut open_backslash = false;
        let mut open_backtick = false;
        let special = ['"', '\'', '\\', '`'];
        let new_vec: Vec<&str> = self.split('\n').collect();

        for (i, next_str) in new_vec.iter().enumerate() {
            if open_double_quote {
                result.push('\n');
            }
            if next_str.is_empty() {
                if open_backslash && i != new_vec.len() - 1 {
                    open_backslash = false;
                }
            }
            let mut chars = next_str.chars().peekable();
            while let Some(ch) = chars.next() {
                match ch {
                    '"' if !open_single_quote && !open_backtick => {
                        open_double_quote = !open_double_quote;
                    }
                    '\'' if !open_double_quote && !open_backtick => {
                        open_single_quote = !open_single_quote;
                    }
                    '`' if !open_double_quote && !open_single_quote => {
                        open_backtick = !open_backtick;
                    }

                    '\\' if !open_backslash && !open_double_quote && !open_single_quote => {
                        open_backslash = true;
                        match chars.peek() {
                            Some(&ch2) => {
                                open_backslash = false;
                                result.push(ch2);
                                chars.next();
                            }
                            None => {}
                        }
                    }
                    '\\' if open_double_quote && !open_single_quote => match chars.peek() {
                        Some(&ch2) => {
                            if special.contains(&ch2) {
                                result.push(ch2);
                            } else {
                                result.push(ch);
                                result.push(ch2);
                            }
                            chars.next();
                        }
                        None => {}
                    },
                    ch if ch.is_whitespace()
                        && !open_double_quote
                        && !open_single_quote
                        && !open_backtick
                        && !open_backslash =>
                    {
                        let le = result.len();
                        if le > 0 {
                            if &result[le - 1..] != " " {
                                result.push(' ');
                            }
                        }
                    }
                    _ if !open_single_quote => {
                        if ch == '\\' {
                            open_backslash = true;
                        } else {
                            open_backslash = false;
                            result.push(ch);
                        }
                    }
                    _ => result.push(ch),
                }
            }
        }

        // println!("result => {:#?}", result);

        let open = open_double_quote || open_single_quote || open_backslash || open_backtick;

        (result, open)
    }
}
pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
