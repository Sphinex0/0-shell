use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::process::exit;

use shell::*;

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
            exit(0)
        }

        let (mut input, mut open_quote) = entry.costum_split();
        if input.is_empty() {
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
                let (input_tmp, open_quote2) = entry.costum_split();
                open_quote = open_quote2;
                input = input_tmp;
                if !open_quote {
                    break;
                }
            }
        }

        if open_quote {
            print_error("Syntax error: Unterminated quoted string");
            continue;
        }

        if entry.split_whitespace().collect::<Vec<_>>().len() != 0 {
            hist.push(entry.clone());
        }

        let command = input[0].as_str();
        let args: Vec<String> = if input.len() > 1 {
            input[1..].to_vec()
        } else {
            Vec::new()
        };
        match command {
            "echo" => {
                echo(&args, &entry);
            }
            "pwd" => {
                pwd(&current_dir);
            }
            "cd" => {
                cd(&args, &mut current_dir, &mut history_current_dir, &home);
            }
            "ls" => {
                ls(&args, &current_dir);
            }
            "cat" => {
                cat(&args, &current_dir);
            }
            "cp" => {
                cp(&args);
            }
            "rm" => {
                rm(&args, &current_dir);
            }
            "mv" => {
                mv(&args);
            }
            "mkdir" => {
                mkdir(&args, &current_dir);
            }
            "history" => {
                history(&hist);
            }
            "exit" => exit(0),
            _ => {
                println!("\x1b[31m Command '<{command}>' not found\x1b[0m")
            }
        }
    }
}
