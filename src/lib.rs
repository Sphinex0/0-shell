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

#[derive(Debug, PartialEq)]
pub enum CommandPart {
    String(String),        // A literal string, e.g., "hello"
    Substitution(Command), // A nested command, e.g., `echo d`
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,           // The command name, e.g., "echo"
    pub args: Vec<CommandPart>, // List of arguments (strings or substitutions)
}

impl Command {
    pub fn add_string(&mut self, word: &String) {
        if word.is_empty() {
            return;
        }
        if self.name.is_empty() {
            self.name = word.clone();
        } else {
            self.args.push(CommandPart::String(word.clone()));
        }
    }
}

pub trait CostumSplit {
    fn custom_split(&self) -> (Command, bool);
}

impl CostumSplit for String {
    fn custom_split(&self) -> (Command, bool) {
        // println!("self => :{self:?}");
        let mut command = Command {
            name: String::new(),
            args: Vec::new(),
        };
        let mut word = String::new();
        let mut state = State::Normal;
        let mut open_backslash = false;
        let mut backtick_str = String::new();

        #[derive(Debug, PartialEq)]
        enum State {
            Normal,
            DoubleQuote,
            SingleQuote,
            Backtick,
        }

        let chs = self.split("\n").collect::<Vec<_>>();
        for (i, line) in chs.iter().enumerate() {
            if matches!(state, State::DoubleQuote | State::SingleQuote) {
                // command.add_string(&"\n".to_string());
                word.push('\n');
            }
            if line.is_empty() {
                if open_backslash && i != chs.len() - 1 {
                    open_backslash = false;
                }
            }
            let mut chars = line.chars().peekable();
            while let Some(ch) = chars.next() {
                match state {
                    State::Normal => {
                        if ch.is_whitespace() && !open_backslash {
                            command.add_string(&word);

                            let le = command.args.len();
                            if le > 0
                                && command.args[le - 1] != CommandPart::String(" ".to_string())
                            {
                                command.add_string(&" ".to_string());
                            }

                            word.clear();
                        } else if ch == '"' && !open_backslash {
                            state = State::DoubleQuote;
                        } else if ch == '\'' && !open_backslash {
                            state = State::SingleQuote;
                        } else if ch == '`' && !open_backslash {
                            command.add_string(&word);
                            word.clear();

                            state = State::Backtick;
                            backtick_str.clear();
                        } else if ch == '\\' {
                            open_backslash = true;
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
                        } else if ch == '\\' {
                            // println!("ff");
                            open_backslash = true;
                            match chars.peek() {
                                Some(&ch2) => {
                                    open_backslash = false;
                                    word.push(ch2);
                                    chars.next();
                                }
                                None => {}
                            }
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
                        } else {
                            word.push(ch);
                        }
                    }
                    State::Backtick => {
                        if ch == '`' {
                            // let (nested_command, _) = backtick_str.custom_split();
                            let (nested_command, _) = backtick_str.custom_split();
                            command.args.push(CommandPart::Substitution(nested_command));
                            println!("gg => :{command:?}");
                            state = State::Normal;
                        } else {
                            backtick_str.push(ch);
                        }
                    }
                }
            }

            // if !word.is_empty() {
            //     command.add_string(&word);
            // }
        }

        if !word.is_empty() {
            command.add_string(&word);
        }

        let open = matches!(state, State::DoubleQuote | State::SingleQuote)
            || open_backslash
            || state == State::Backtick;
        (command, open)
    }
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
