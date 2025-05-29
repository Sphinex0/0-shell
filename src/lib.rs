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
        if word.len() == 0 {
            return;
        }
        if self.name.is_empty() {
            self.name = word.clone()
        } else {
            self.args.push(CommandPart::String(word.clone()));
        }
    }
}

pub trait CostumSplit {
    fn costum_split(&self) -> (Command, bool);
}

impl CostumSplit for String {
    fn costum_split(&self) -> (Command, bool) {
        let mut command = Command {
            name: String::new(),
            args: Vec::new(),
        };
        let mut word = String::new();
        let mut main_command = false;

        // commands.push(Vec::new());

        let mut backtick_result: Vec<String> = Vec::new();
        let mut backtick_arg: String = String::new();

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
                '`' => {
                    open_backtick_quote = !open_backtick_quote;

                    if open_backtick_quote {
                        command.args.push(CommandPart::Substitution(Command {
                            name: String::new(),
                            args: Vec::new(),
                        }));
                    } else {
                        let last_index = command.args.len() - 1;

                        if let CommandPart::Substitution(sub_command) =
                            &mut command.args[last_index]
                        {
                            sub_command.add_string(&word);
                        };
                        word.clear();
                    }
                }
                '\\' => {
                    // if !open_backslash_quote {
                    //     open_backslash_quote = true;
                    //     match chars.peek() {
                    //         Some(ch2) => {
                    //             open_backslash_quote = false;
                    //             arg.push(*ch2);
                    //             chars.next();
                    //         }
                    //         _ => {}
                    //     }
                    // }
                }

                _ => {
                    if ch.is_whitespace()
                        && !(open_double_quote || open_single_quote || open_backslash_quote)
                    {
                        if open_backtick_quote {
                            let last_index = command.args.len() - 1;
                            if let CommandPart::Substitution(sub_command) =
                                &mut command.args[last_index]
                            {
                                sub_command.add_string(&word);
                            };
                        } else {
                            command.add_string(&word);

                            let le = command.args.len();
                            if le > 0
                                && command.args[le - 1] != CommandPart::String(" ".to_string())
                            {
                                command.args.push(CommandPart::String(" ".to_string()));
                            }
                        }

                        word.clear();
                    } else {
                        word.push(ch);
                    }
                }
            }
        }

        /* flush anything left */
        command.add_string(&word);

        // println!("{:?}", result);

        let open =
            open_double_quote || open_single_quote || open_backslash_quote || open_backtick_quote;
        // println!("command : {command:#?}");
        (command, open)
    }
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m {}\x1b[0m", message)
}
