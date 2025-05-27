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

pub trait CostumSplit {
    fn costum_split(&self) -> (Vec<String>, bool);
}

impl CostumSplit for String {
    fn costum_split(&self) -> (Vec<String>, bool) {
        let mut result: Vec<String> = Vec::new();
        let mut arg: String = String::new();
        let mut open_double_quote = false;
        let mut open_single_quote = false;
        let mut open_backslash_quote = false;
        let mut open_backtick_quote = false;

        let special = ['"', '\'', '\\', '`'];

        let mut chars = self.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '"' => open_double_quote = !open_double_quote,
                '\'' => open_single_quote = !open_single_quote,
                '`' => open_backtick_quote = !open_backtick_quote,
                '\\' => {
                    if !open_backslash_quote {
                        open_backslash_quote = true;
                        match chars.peek() {
                            Some(ch2) => {
                                open_backslash_quote = false;
                                arg.push(*ch2);
                                chars.next();
                            }
                            _ => {
                                
                            }
                        }
                    }
                }

                _ => {
                    if ch.is_whitespace() {
                        result.push(arg);
                        arg = String::new();
                    } else {
                        arg.push(ch);
                    }
                }
            }
        }

        if !arg.is_empty() {
            result.push(arg);
        }

        println!("{:?}", result);

        let open =
            open_double_quote || open_single_quote || open_backslash_quote || open_backtick_quote;

        (result, open)
    }
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
