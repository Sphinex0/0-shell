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
    fn custom_split(&self) -> (Vec<String>, bool);
}

impl CustomSplit for String {
    fn custom_split(&self) -> (Vec<String>, bool) {
        let mut result: Vec<String> = Vec::new();
        let mut arg: String = String::new();
        let mut open_double_quote = false;
        let mut open_single_quote = false;
        let mut open_backslash = false;
        let mut open_backtick = false;

        let new_vec: Vec<&str> = self.split('\n').collect();

        for (i, next_str) in new_vec.iter().enumerate() {
            if open_double_quote {
                // open_double_quote = false;
                arg.push('\n');
            }
            if next_str.is_empty() {
                if open_backslash && i != new_vec.len() - 1 {
                    open_backslash = false;
                }

                // arg.push('\n');
            }
            let mut chars = next_str.chars().peekable();
            while let Some(ch) = chars.next() {
                match ch {
                    '"' if !open_single_quote && !open_backtick => {
                        open_double_quote = !open_double_quote;
                        // arg.push(ch);
                    }
                    '\'' if !open_double_quote && !open_backtick => {
                        open_single_quote = !open_single_quote;
                        // arg.push(ch);
                    }
                    '`' if !open_double_quote && !open_single_quote => {
                        open_backtick = !open_backtick;
                        // arg.push(ch);
                    }
                    '\\' if !open_backslash && !open_double_quote => {
                        open_backslash = true;
                        match chars.peek() {
                            Some(&ch2) => {
                                open_backslash = false;
                                // arg.push(ch);
                                arg.push(ch2);
                                chars.next();
                            }
                            None => {
                                // arg.push(ch);
                            }
                        }
                    }
                    ch if ch.is_whitespace()
                        && !open_double_quote
                        && !open_single_quote
                        && !open_backtick
                        && !open_backslash =>
                    {
                        if !arg.is_empty() {
                            result.push(arg.clone());
                            result.push(" ".to_string());
                            arg.clear();
                        }
                    }
                    _ => {
                        if open_backslash {
                            println!("ch => {ch}");
                            open_backslash = !open_backslash;
                            let le: usize = result.len();
                            if le > 0 {
                                println!("sd");
                                result[le - 1].push(ch);
                            } else {
                                arg.push(ch);
                            }
                        } else {
                            arg.push(ch);
                        }
                    }
                }
            }

            // println!("arg =>{arg:?}");

            if !arg.is_empty() {
                result.push(arg.clone());
                arg.clear();
            }
        }

        // println!("result => {:?}", result);

        let open = open_double_quote || open_single_quote || open_backslash || open_backtick;

        (result, open)
    }
}
pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}

// pub trait CostumSplit {
//     fn costum_split(&self) -> (Vec<String>, bool);
// }

// impl CostumSplit for String {
//     fn costum_split(&self) -> (Vec<String>, bool) {
//         let mut result: Vec<&str> = Vec::new();
//         let mut arg: String = String::new();
//         let mut open_double_quote = false;
//         let mut open_single_quote = false;
//         let mut open_backslash_quote = false;
//         let mut open_backtick_quote = false;

//         let special = ['"', '\'', '\\', '`'];

//         let new_vec = self.split("\n").collect::<Vec<_>>();

//         for next_str in new_vec {
//             let mut chars = next_str.chars().peekable();
//             while let Some(ch) = chars.next() {
//                 match ch {
//                     '"' => open_double_quote = !open_double_quote,
//                     '\'' => open_single_quote = !open_single_quote,
//                     '`' => open_backtick_quote = !open_backtick_quote,
//                     '\\' => {
//                         if !open_backslash_quote {
//                             open_backslash_quote = true;
//                             match chars.peek() {
//                                 Some(ch2) => {
//                                     open_backslash_quote = false;
//                                     arg.push(*ch2);
//                                     chars.next();
//                                 }
//                                 _ => {}
//                             }
//                         }
//                     }

//                     _ => {
//                         if ch.is_whitespace() {
//                             result.push(&arg);
//                             arg = String::new();
//                         } else {
//                             arg.push(ch);
//                         }
//                     }
//                 }
//             }

//             if !arg.is_empty() {
//                 result.push(&arg);
//             }
//         }

//         println!("{:?}", result);

//         let open =
//             open_double_quote || open_single_quote || open_backslash_quote || open_backtick_quote;

//         let mut res: Vec<String> = vec![];

//         for a in result {
//             res.push(a.to_string());
//         }

//         (res, open)
//     }
// }
