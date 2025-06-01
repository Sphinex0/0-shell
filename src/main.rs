use shell::*;
use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::process::exit;

use std::path::PathBuf;

fn exec_command(
    command: &str,
    args: &[String],
    current_dir: &mut PathBuf,
    history_current_dir: &mut PathBuf,
    hist: &Vec<String>,
    home: &PathBuf,
) -> Option<String> {
    match command {
        "echo" => Some(echo(args)),
        "pwd" => Some(pwd(current_dir)),
        "cd" => {
            cd(args, current_dir, history_current_dir, home);
            None
        }
        // "mv" => {
        //     mv(&args);
        //     None
        // }
        // "cp" => {
        //     cp(&args);
        //     None
        // }
        "ls" => Some(ls(&args, &current_dir)),
        "cat" => Some(cat(args, current_dir)),
        "rm" => {
            rm(args, current_dir);
            None
        }
        "mkdir" => {
            mkdir(args, current_dir);
            None
        }
        "history" => Some(history(hist)),
        "exit" => exit(0),
        "clear" => {
            println!("\x1Bc");
            None
        }
        _ => {
            print_error(&format!("Command <{}> not found", command));
            None
        }
    }
}

fn main() {
    let mut current_dir = current_dir().unwrap();
    let mut history_current_dir = current_dir.clone();
    let mut hist: Vec<String> = Vec::new();
    let home = match home_dir() {
        Some(p) => p,
        None => {
            print_error("Impossible to get your home dir!");
            return;
        }
    };

    loop {
        let address = match current_dir.strip_prefix(&home) {
            Ok(p) => "\x1b[1;31m~\x1b[1;32m/".to_string() + &p.display().to_string(),
            Err(_) => current_dir.display().to_string(),
        };

        print!("\x1b[1;33m➜  \x1b[1;32m{} \x1b[33m$ \x1b[0m", address);
        std::io::stdout().flush().unwrap();
        let mut entry = String::new();
        let size = stdin().read_line(&mut entry).unwrap();
        if size == 0 {
            println!();
            exit(0);
        }

        let (mut command, mut open_quote, mut quite1) = entry.custom_split();
        if quite1 {
            continue;
        }

        if open_quote {
            loop {
                print!("\x1b[33m> \x1b[0m");
                let mut input_tmp = String::new();
                std::io::stdout().flush().unwrap();
                let size = stdin().read_line(&mut input_tmp).unwrap();
                if size == 0 {
                    println!();
                    break;
                }
                entry.push_str(&input_tmp);
                let (input_tmp, open_quote2, quite) = entry.custom_split();
                if quite {
                    quite1 = quite;
                    break;
                }
                open_quote = open_quote2;
                command = input_tmp;
                if !open_quote {
                    break;
                }
            }
        }

        if quite1 {
            continue;
        }

        // println!("command => {:?}", command);

        if open_quote {
            print_error("Syntax error: Unterminated quoted string");
            continue;
        }

        // First Pass: Process command and substitutions
        let mut first_pass_res: Vec<String> = Vec::new();
        if !command.name.is_empty() {
            first_pass_res.push(command.name.clone());
        }
        for arg in command.args {
            match arg {
                CommandPart::String(arg) => {
                    if !arg.is_empty() {
                        first_pass_res.push(arg);
                    }
                }
                CommandPart::Substitution(sub_command) => {
                    if !sub_command.name.is_empty() {
                        let mut sub_args: Vec<String> = Vec::new();
                        for sub_arg in sub_command.args {
                            // if let CommandPart::String(sub_arg_str) = sub_arg {
                            //     sub_args.push(sub_arg_str);
                            // }

                            ///////////////////
                            match sub_arg {
                                CommandPart::String(sub_arg_str) => sub_args.push(sub_arg_str),
                                CommandPart::Substitution(sub_command) => {
                                    if !sub_command.name.is_empty() {
                                        let sub_args2: Vec<String> = Vec::new();
                                        for sub_arg in sub_command.args {
                                            if let CommandPart::String(sub_arg_str) = sub_arg {
                                                sub_args.push(sub_arg_str);
                                            }
                                        }
                                        let output = exec_command(
                                            &sub_command.name,
                                            &sub_args2,
                                            &mut current_dir,
                                            &mut history_current_dir,
                                            &hist,
                                            &home,
                                        );
                                        if let Some(out) = output {
                                            sub_args.push(out)
                                        }
                                        // sub_args.push(output.unwrap_or_default());
                                    }
                                }
                            }
                            ///////////////////
                        }
                        let output = exec_command(
                            &sub_command.name,
                            &sub_args,
                            &mut current_dir,
                            &mut history_current_dir,
                            &hist,
                            &home,
                        );
                        // first_pass_res.push(output.unwrap_or_default());
                        if let Some(out) = output {
                            first_pass_res.push(out)
                        }
                    }
                }
            }
        }

        // Second Pass: Execute the final command
        if !first_pass_res.is_empty() && !first_pass_res[0].is_empty() {
            let command = &first_pass_res[0];
            let args = &first_pass_res[1..];
            let output = exec_command(
                command,
                args,
                &mut current_dir,
                &mut history_current_dir,
                &hist,
                &home,
            );
            if let Some(output) = output {
                println!("{}", output);
            }
        }

        // Add to history if entry has non-whitespace characters
        if entry.split_whitespace().collect::<Vec<_>>().len() != 0 {
            hist.push(entry.clone());
        }
    }
}
