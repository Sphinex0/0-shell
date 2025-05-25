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
        let mut arg = String::new();
        let mut open_quote = false;


        // let mut chars = self.chars().peekable();
        // while let Some(ch) = chars.next() {
        //     println!("ch: {ch}");
        //     match ch {
        //         '\\' => match chars.next() {
        //             Some(ch2) => match (ch2, open_quote) {
        //                 ('n', true) => arg.push('\n'),
        //                 ('r', true) => arg.push('\r'),
        //                 ('t', true) => arg.push('\t'),
        //                 ('a', true) => arg.push(7 as char),
        //                 ('b', true) => arg.push(8 as char),
        //                 ('\\', false) => match chars.next() {
        //                     Some(ch3) => match ch3 {
        //                         'n' => arg.push('\n'),
        //                         'r' => arg.push('\r'),
        //                         't' => arg.push('\t'),
        //                         'a' => arg.push(7 as char),
        //                         'b' => arg.push(8 as char),
        //                         _ => {
        //                             arg.push_str(&format!("\\{ch3}"));
        //                         }
        //                     },
        //                     None => arg.push('\\'),
        //                 },
        //                 (a, true) => {
        //                     // println!("dddddd => {:?}", a);
        //                     arg.push(a);
        //                 }
        //                 (b, false) => {
        //                     if b.is_whitespace() {
        //                         result.push(arg);
        //                         arg = String::new();
        //                     } else {
        //                         arg.push(b);
        //                     }
        //                 }
        //             },
        //             None => {}
        //         },
        //         '"' => {
        //             open_quote = !open_quote;
        //         }
        //         _ => {
        //             if ch.is_whitespace() && !open_quote {
        //                 result.push(arg);
        //                 arg = String::new();
        //             } else {
        //                 arg.push(ch);
        //             }
        //         }
        //     }
        // }

        // originale code 
        for (i, ch) in self.char_indices() {
            if (ch == '"' && i == 0)
                || (ch == '"' && self[i - 1..i].chars().next().unwrap_or(' ') != '\\')
            {
                open_quote = !open_quote;
                continue;
            }
            if ch.is_whitespace() && !open_quote {
                if !arg.is_empty() {
                    result.push(arg);
                    arg = String::new();
                }
            } else {
                arg.push(ch)
            }
        }



        if !arg.is_empty() {
            result.push(arg);
        }

        // println!("{:?}", result);

        (result, open_quote)
    }
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
