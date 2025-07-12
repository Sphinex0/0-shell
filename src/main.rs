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
    last_command_staus: &Option<i32>,
) -> (Option<(String, bool)>, i32) {
    match command {
        "echo" => (Some(echo(args)), 0),
        "pwd" => (Some((pwd(current_dir), true)), 0),
        "cd" => (None, cd(args, history_current_dir, current_dir, home)),
        "mv" => {
            mv(&args);
            (None, 0)
        }
        "cp" => {
            cp(&args);
            (None, 0)
        }
        "ls" => (Some((ls(&args, &current_dir), true)), 0),
        "cat" => (Some((cat(args, current_dir), false)), 0),
        "rm" => {
            rm(args, current_dir);
            (None, 0)
        }
        "mkdir" => {
            mkdir(args, current_dir);
            (None, 0)
        }
        "history" => (Some((history(hist), true)), 0),
        "exit" => {
            if args.len() == 0 {
                match last_command_staus {
                    Some(code) => exit(*code),
                    None => exit(0),
                }
            } else {
                match args[0].parse::<i32>() {
                    Ok(code) => exit(code),
                    Err(_) => {
                        print_error("exit: Illegal number: ");
                        (None, 2)
                    }
                }
            }
        }
        "clear" => {
            println!("\x1Bc");
            (None, 0)
        }
        _ => {
            print_error(&format!("Command <{}\x1b[31m> not found", command));
            (None, 127)
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

    \x1b[1;0m"
    );

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

    let mut last_command_staus: Option<i32> = None;

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
                    // println!();
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

        // println!("command => {:#?}", command);

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
            &last_command_staus,
        );
        last_command_staus = Some(output.1);
        if let Some((output, newline)) = output.0 {
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

