use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::process::exit;

use shell::*;

fn main() {
    let mut current_dir = current_dir().unwrap();
    let mut history = current_dir.clone();
    loop {
        print!(
            "\x1b[31m~\x1b[32m{} \x1b[33m$ \x1b[0m",
            current_dir.display()
        );
        std::io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            buf
        };
        if input.len() == 0 {
            println!();
            exit(0)
        }
        
        let input = input.split_whitespace().collect::<Vec<_>>();
        if input.is_empty() {
            continue;
        }
        let command = input[0];
        let args: Vec<&str> = if input.len() > 1 {
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
                cd(&args, &mut current_dir, &mut history);
            }
            "ls" => {
                ls(&args, &current_dir);
            }
            "cat" => {
                cat(&args);
            }
            "cp" => {
                cp(&args);
            }
            "rm" => {
                rm(&args);
            }
            "mv" => {
                mv(&args);
            }
            "mkdir" => {
                mkdir(&args);
            }
            "exit" => exit(0),
            _ => {
                println!("\x1b[31m Command '<{command}>' not found\x1b[0m")
            }
        }
    }
}
