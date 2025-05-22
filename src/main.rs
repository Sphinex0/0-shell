use std::io::Write;
use std::io::stdin;
use std::process::exit;

use shell::*;

fn main() {
    loop {
        print!("$ ");
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
            "cd" => {
                cd(args);
            }
            "ls" => {
                ls(args);
            }
            "cat" => {
                cat(args);
            }
            "cp" => {
                cp(args);
            }
            "rm" => {
                rm(args);
            }
            "mv" => {
                mv(args);
            }
            "mkdir" => {
                mkdir(args);
            }
            "exit" => exit(0),
            _ => {}
        }
    }
}