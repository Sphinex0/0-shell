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
) -> Option<(String, bool)> {
    match command {
        "echo" => Some(echo(args)),
        "pwd" => Some((pwd(current_dir), true)),
        "cd" => {
            // cd(args, home);
            cd(args, history_current_dir, current_dir, home);
            None
        }
        "mv" => {
            mv(&args);
            None
        }
        "cp" => {
            cp(&args);
            None
        }
        "ls" => Some((ls(&args, &current_dir), true)),
        "cat" => Some((cat(args, current_dir), true)),
        "rm" => {
            rm(args, current_dir);
            None
        }
        "mkdir" => {
            mkdir(args, current_dir);
            None
        }
        "history" => Some((history(hist), true)),
        "exit" => exit(0),
        "clear" => {
            println!("\x1Bc");
            None
        }
        _ => {
            print_error(&format!("Command <{}\x1b[31m> not found", command));
            None
        }
    }
}

fn main() {
    // set_current_dir(path)
    let mut history_current_dir = current_dir().unwrap();
    let mut current_dir = current_dir().unwrap();
    let mut hist: Vec<String> = Vec::new();
    let home = match home_dir() {
        Some(p) => p,
        None => {
            print_error("Impossible to get your home dir!");
            return;
        }
    };

    loop {
        // let mut current_dir = current_dir().unwrap();
        let address = match current_dir.strip_prefix(&home) {
            Ok(p) => "\x1b[1;31m~\x1b[1;36m/".to_string() + &p.display().to_string(),
            Err(_) => current_dir.display().to_string(),
        };

        print!("\x1b[1;33m➜  \x1b[1;36m{} \x1b[33m$ \x1b[0m", address);
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
                                        if let Some((out, _)) = output {
                                            println!("out=>{out}");
                                            // out.split(" ")
                                            //     .for_each(|arg| sub_args.push(arg.to_string()));
                                            let words = out.split(" ").collect::<Vec<_>>();
                                            for (i, word) in words.iter().enumerate() {
                                                if !word.is_empty() {
                                                    sub_args.push(word.to_string());
                                                }
                                                if i > 0
                                                    && i != word.len().saturating_sub(1)
                                                    && words[i.saturating_sub(1)] != ""
                                                {
                                                    sub_args.push(" ".to_string());
                                                }
                                            }
                                            // sub_args.push(out)
                                        } else {
                                            sub_args.pop();
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
                        if let Some((out, _)) = output {
                            // out.split(" ")
                            //     .for_each(|arg| first_pass_res.push(arg.to_string()));
                            let words = out.split(" ").collect::<Vec<_>>();
                            for (i, word) in words.iter().enumerate() {
                                if !word.is_empty() {
                                    first_pass_res.push(word.to_string());
                                }
                                if i > 0
                                    && i != word.len().saturating_sub(1)
                                    && words[i.saturating_sub(1)] != ""
                                {
                                    first_pass_res.push(" ".to_string());
                                }
                            }
                            // first_pass_res.push(out)
                        } else {
                            first_pass_res.pop();
                        }
                    }
                }
            }
        }

        // println!("first_pass_res =>{:?}", first_pass_res);

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
            if let Some((output, newline)) = output {
                if newline {
                    println!("{}", output);
                } else {
                    print!("{}", output);
                }
            }
        }

        // Add to history if entry has non-whitespace characters
        if entry.split_whitespace().collect::<Vec<_>>().len() != 0 {
            hist.push(entry.clone());
        }
    }
}