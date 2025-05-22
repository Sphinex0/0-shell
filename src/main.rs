use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::process::exit;

use shell::*;

fn main() {
    let mut current_dir = current_dir().unwrap();
    loop {
        print!("\x1b[32m{} \x1b[33m$ \x1b[0m", current_dir.display());
        std::io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            buf
        };
        
        let input = input.split_whitespace().collect::<Vec<_>>();
        let command = input[0];
        let args = &input[1..];
        match command {
            "echo" => {
                echo(args);
            }
            "pwd" => {
                pwd(&mut current_dir);
            }
            "cd" => {
                cd(args);
            }
            "ls" => {
                ls(args);
            }
            "cat" => {
                cat(args, &current_dir);
            }
            "cp" => {
                cp(args);
            }
            "rm" => {
                rm(args, &current_dir);
            }
            "mv" => {
                mv(args);
            }
            "mkdir" => {
                mkdir(args, &current_dir);
            }
            "exit" => exit(0),
            _ => {
                println!("\x1b[31m Command '<{command}>' not found\x1b[0m")
            }
        }
    }
}
