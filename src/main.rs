use ctrlc;
use shell::*;
use std::env::*;
use std::io::Write;
use std::io::stdin;
use std::path::PathBuf;
use std::process::exit;

fn exec_command(
    command: &str,
    args: &[String],
    current_dir: &mut PathBuf,
    history_current_dir: &mut PathBuf,
    hist: &Vec<String>,
    home: &PathBuf,
    last_command_staus: i32,
) -> i32 {
    match command {
        "echo" => echo(args),
        "pwd" => pwd(current_dir),
        "cd" => cd(args, history_current_dir, current_dir, home),
        "mv" => mv(&args),
        "cp" => cp(&args),
        "ls" => ls(&args, &current_dir),
        "cat" => cat(args, current_dir),
        "rm" => rm(args, current_dir),
        "mkdir" => mkdir(args, current_dir),
        "history" => history(hist),
        "exit" => {
            if args.len() == 0 {
                exit(last_command_staus);
            } else {
                match args[0].parse::<i32>() {
                    Ok(code) => exit(code),
                    Err(_) => {
                        print_error("exit: Illegal number: ");
                        2
                    }
                }
            }
        }
        "clear" => {
            println!("\x1Bc");
            0
        }
        _ => {
            print_error(&format!("Command <{}\x1b[31m> not found", command));
            127
        }
    }
}

fn main() {
    println!(
        "\x1b[1;31m
     ██████╗     ███████╗██╗  ██╗███████╗██╗     ██╗     
    ██╔═████╗    ██╔════╝██║  ██║██╔════╝██║     ██║     
    ██║██╔██║    ███████╗███████║█████╗  ██║     ██║     
    ████╔╝██║    ╚════██║██╔══██║██╔══╝  ██║     ██║     
    ╚██████╔╝    ███████║██║  ██║███████╗███████╗███████╗
    ╚═════╝     ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
    ಠ_ಠ ѕρнιηєχ  | к!ℓℓṳ ✘_✘ | ṭѧ9ѧʏṭ ◔_◔ |  GᑌTᔕ (ಠ‿ಠ)

    \x1b[1;0m"
    );

    // set_current_dir(path)
    let mut history_current_dir = current_dir().unwrap_or(PathBuf::from("/"));
    let mut current_dir = history_current_dir.clone();
    let mut hist: Vec<String> = Vec::new();
    let home = match home_dir() {
        Some(p) => p,
        None => {
            print_error("Impossible to get your home dir!");
            current_dir.clone()
        }
    };

    let mut last_command_staus = 0;
    if let Err(_) = ctrlc::set_handler(|| {}) {
        print_error("Error setting Ctrl+C handler");
    };


    loop {
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
            last_command_staus,
        );
        last_command_staus = output;

        // Add to history if entry has non-whitespace characters
        if entry.split_whitespace().collect::<Vec<_>>().len() != 0 {
            hist.push(entry.clone());
        }
    }
}
