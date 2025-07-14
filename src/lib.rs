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


#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub name: String,      // The command name, e.g., "echo"
    pub args: Vec<String>, // List of arguments
}

impl Command {
    pub fn add_string(&mut self, word: &String) {
        if word.is_empty() {
            return;
        }
        if self.name.is_empty() {
            self.name = word.clone();
        } else {
            self.args.push(word.clone());
        }
    }

    pub fn add_string_whatever(&mut self, word: &String) {
        if self.name.is_empty() {
            self.name = word.clone();
        } else {
            self.args.push(word.clone());
        }
    }
}

pub trait CostumSplit {
    fn custom_split(&self) -> (Command, bool);
}

impl CostumSplit for String {
    fn custom_split(&self) -> (Command, bool) {
        let mut command = Command {
            name: String::new(),
            args: Vec::new(),
        };
        let mut word = String::new();
        let mut state = State::Normal;
        let mut open_backslash = false;

        #[derive(Debug, PartialEq)]
        enum State {
            Normal,
            DoubleQuote,
            SingleQuote,
        }

        let chs = self.split("\n").collect::<Vec<_>>();
        for (i, line) in chs.iter().enumerate() {
            if state != State::Normal && !open_backslash {
                word.push('\n');
            }
            if open_backslash && i != chs.len() - 1 {
                open_backslash = false;
            }
            let mut chars = line.chars().peekable();
            while let Some(ch) = chars.next() {
                match state {
                    State::Normal => {
                        if ch == '\\' && !open_backslash {
                            open_backslash = true;
                        } else if ch.is_whitespace() && !open_backslash {
                            command.add_string(&word);

                            // let le = command.args.len();
                            // if le > 0
                            //     && command.args[le - 1] != " ".to_string()
                            // {
                            //     command.add_string(&" ".to_string());
                            // }

                            word.clear();
                        } else if ch == '"' && !open_backslash {
                            state = State::DoubleQuote;
                        } else if ch == '\'' && !open_backslash {
                            state = State::SingleQuote;
                        } else {
                            if open_backslash {
                                word.push(ch);
                                open_backslash = false;
                            } else {
                                word.push(ch);
                            }
                        }
                    }
                    State::DoubleQuote => {
                        if ch == '"' && !open_backslash {
                            state = State::Normal;
                            if let Some(ch2) = chars.peek() {
                                if ch2.is_whitespace() {
                                    command.add_string_whatever(&word);
                                    word.clear();
                                    chars.next();
                                }
                            }
                        } else if ch == '\\' && !open_backslash {
                            open_backslash = true;
                        } else {
                            if open_backslash {
                                if ['"', '\\', '`', '$'].contains(&ch) {
                                    word.push(ch);
                                } else {
                                    word.push('\\');
                                    word.push(ch);
                                }
                                open_backslash = false;
                            } else {
                                word.push(ch);
                            }
                        }
                    }
                    State::SingleQuote => {
                        if ch == '\'' {
                            state = State::Normal;
                            if let Some(ch2) = chars.peek() {
                                if ch2.is_whitespace() {
                                    command.add_string_whatever(&word);
                                    word.clear();
                                    chars.next();
                                }
                            }
                        } else {
                            word.push(ch);
                        }
                    }
                }
            }
        }

        if !word.is_empty() {
            command.add_string(&word);
        }

        let open = matches!(state, State::DoubleQuote | State::SingleQuote) || open_backslash;
        (command, open)
    }
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
