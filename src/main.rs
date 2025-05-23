use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::process::exit;

use shell::*;

fn main() {
    let mut current_dir = current_dir().unwrap();
    let mut history_current_dir = current_dir.clone();
    let mut hist:Vec<&str> = Vec::new();
    loop {
        print!(
            "\x1b[31m~\x1b[32m{} \x1b[33m$ \x1b[0m",
            current_dir.display()
        );
        std::io::stdout().flush().unwrap();
        let mut entry = String::new();
        stdin().read_line(&mut entry).unwrap();

        let (mut input, open_quote) = entry.costum_split();
        if input.is_empty() {
            continue;
        }
        if open_quote {
            loop {
                print!("\x1b[33m> \x1b[0m");
                let mut input_tmp = String::new();
                std::io::stdout().flush().unwrap();
                stdin().read_line(&mut input_tmp).unwrap();
                entry.push_str(&input_tmp);
                let (input_tmp, open_quote) = entry.costum_split();
                input = input_tmp;
                if !open_quote {
                    break;
                }
            }
        }
        
        let command = input[0].as_str();
        let args: Vec<String> = if input.len() > 1 {
            input[1..].to_vec()
        } else {
            Vec::new()
        };
        match command {
            "echo" => {
                echo(&args);
            }
            "pwd" => {
                pwd(&current_dir);
            }
            "cd" => {
                cd(&args, &mut current_dir, &mut history_current_dir);
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
                println!("\x1b[31m Command '<{entry}>' not found\x1b[0m")
            }
        }
    }
}
