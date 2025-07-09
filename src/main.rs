use shell::*;
use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::process::exit;

use ctrlc;
use std::sync::mpsc::channel;

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
    let (tx, _rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!(
    "\x1b[1;31m
     ██████╗     ███████╗██╗  ██╗███████╗██╗     ██╗     
    ██╔═████╗    ██╔════╝██║  ██║██╔════╝██║     ██║     
    ██║██╔██║    ███████╗███████║█████╗  ██║     ██║     
    ████╔╝██║    ╚════██║██╔══██║██╔══╝  ██║     ██║     
    ╚██████╔╝    ███████║██║  ██║███████╗███████╗███████╗
    ╚═════╝     ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝

    \x1b[1;0m");

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

        let (mut command, mut open_quote) = entry.custom_split();
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
                let (input_tmp, open_quote2) = entry.custom_split();
                open_quote = open_quote2;
                command = input_tmp;
                if !open_quote {
                    break;
                }
            }
        }

        if command.name.is_empty() {
            continue;
        }

        // println!("command => {:?}", command);

        if open_quote {
            print_error("Syntax error: Unterminated quoted string");
            continue;
        }

        let output = exec_command(
            &command.name,
            &command.args,
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

        // Add to history if entry has non-whitespace characters
        if entry.split_whitespace().collect::<Vec<_>>().len() != 0 {
            hist.push(entry.clone());
        }
    }
}
