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
        for ch in self.chars() {
            if ch == '"' {
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

        (result, open_quote)
    }
}
