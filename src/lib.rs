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

<<<<<<< HEAD
#[derive(Debug, PartialEq, Clone)]
pub enum CommandPart {
    String(String),        // A literal string, e.g., "hello"
    Substitution(Command), // A nested command, e.g., `echo d`
}
=======

>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub name: String,           // The command name, e.g., "echo"
<<<<<<< HEAD
    pub args: Vec<CommandPart>, // List of arguments (strings or substitutions)
=======
    pub args: Vec<String>, // List of arguments (strings or substitutions)
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
}

impl Command {
    pub fn add_string(&mut self, word: &String) {
        if word.is_empty() {
            return;
        }
        if self.name.is_empty() {
            self.name = word.clone();
        } else {
<<<<<<< HEAD
            self.args.push(CommandPart::String(word.clone()));
        }
    }
    pub fn add_argument(&mut self, word: &Command) {
        self.args.push(CommandPart::Substitution(word.clone()));
    }
=======
            self.args.push(word.clone());
        }
    }
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
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
<<<<<<< HEAD
        let mut open_backtick = false;
        let mut backtick_str = String::new();
=======
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76

        #[derive(Debug, PartialEq)]
        enum State {
            Normal,
            DoubleQuote,
            SingleQuote,
        }

        let chs = self.split("\n").collect::<Vec<_>>();
        for (i, line) in chs.iter().enumerate() {
<<<<<<< HEAD
            if open_backtick {
                backtick_str.push('\n');
            }
=======

>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
            if matches!(state, State::DoubleQuote | State::SingleQuote) {
                word.push('\n');
            }
            if open_backslash && i != chs.len() - 1 {
                open_backslash = false;
            }
            let mut chars = line.chars().peekable();
            while let Some(ch) = chars.next() {
                match state {
                    State::Normal => {
<<<<<<< HEAD
                        if open_backtick && ch != '`' {
                            backtick_str.push(ch);
=======
                        if ch == '\\' && !open_backslash {
                            open_backslash = true;
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
                        } else if ch.is_whitespace() && !open_backslash {
                            command.add_string(&word);

                            let le = command.args.len();
                            if le > 0
<<<<<<< HEAD
                                && command.args[le - 1] != CommandPart::String(" ".to_string())
=======
                                && command.args[le - 1] != " ".to_string()
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
                            {
                                command.add_string(&" ".to_string());
                            }

                            word.clear();
                        } else if ch == '"' && !open_backslash {
                            state = State::DoubleQuote;
                        } else if ch == '\'' && !open_backslash {
                            state = State::SingleQuote;
<<<<<<< HEAD
                        } else if ch == '`' && !open_backslash && !open_backtick {
                            command.add_string(&word);
                            word.clear();

                            open_backtick = true;
                            backtick_str.clear();
                        } else if ch == '`' && !open_backslash && open_backtick {
                            // println!("res=>{backtick_str:?}");
                            let (nested_command, _) = backtick_str.custom_split();
                            command.add_argument(&nested_command);
                            open_backtick = false;
                        } else if ch == '\\' {
                            open_backslash = true;
=======
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
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
<<<<<<< HEAD
                        if open_backtick && ch != '`' {
                            backtick_str.push(ch);
                        } else if ch == '"' && !open_backslash {
                            state = State::Normal;
                        } else if ch == '\\' {
                            open_backslash = true;
                        } else if ch == '`' && !open_backslash && !open_backtick {
                            command.add_string(&word);
                            word.clear();

                            open_backtick = true;
                            backtick_str.clear();
                        } else if ch == '`' && !open_backslash && open_backtick {
                            let (nested_command, _) = backtick_str.custom_split();
                            command.add_argument(&nested_command);
                            open_backtick = false;
                        } else {
                            if open_backslash {
                                if ['"', '\\', '`', '$'].contains(&ch) {
                                    word.push(ch);
                                } else {
                                    word.push('\\');
                                    word.push(ch);
                                }
                                open_backslash = false;
                            } else if open_backtick {
                                backtick_str.push(ch);
                            } else {
                                word.push(ch);
                            }
=======
                         if ch == '"' && !open_backslash {
                            state = State::Normal;
                        } else if ch == '\\' && !open_backslash {
                            open_backslash = true;
                        }else {
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
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
                        }
                    }
                    State::SingleQuote => {
                        if ch == '\'' {
                            state = State::Normal;
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

        let open = matches!(state, State::DoubleQuote | State::SingleQuote)
<<<<<<< HEAD
            || open_backslash
            || open_backtick;
=======
            || open_backslash;
>>>>>>> cdcef777dd12f388bd9bb38da800cd9060b03a76
        (command, open)
    }
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
