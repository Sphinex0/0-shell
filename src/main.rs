// use std::env::*;
// use std::io::Write;
// use std::io::stdin;
// use std::process::exit;

// use shell::*;

// fn main() {
//     let mut current_dir = current_dir().unwrap();
//     let mut history_current_dir = current_dir.clone();
//     let mut hist: Vec<String> = Vec::new();
//     let home = match home_dir() {
//         Some(p) => p,
//         None => {
//             print_error("Impossible to get your home dir!");
//             return;
//         }
//     };

//     loop {
//         let address = match current_dir.strip_prefix(&home) {
//             Ok(p) => "\x1b[1;31m~\x1b[1;32m/".to_string() + &p.display().to_string(),
//             Err(_) => current_dir.display().to_string(),
//         };

//         print!("\x1b[1;33m➜  \x1b[1;32m{} \x1b[33m$ \x1b[0m", address);

//         std::io::stdout().flush().unwrap();
//         let mut entry = String::new();
//         let size = stdin().read_line(&mut entry).unwrap();
//         // entry.pop();
//         if size == 0 {
//             println!();
//             exit(0)
//         }

//         let (mut command, mut open_quote) = entry.custom_split();

//         if open_quote {
//             loop {
//                 print!("\x1b[33m> \x1b[0m"); // >
//                 let mut input_tmp = String::new();
//                 std::io::stdout().flush().unwrap();
//                 let size = stdin().read_line(&mut input_tmp).unwrap();
//                 // input_tmp.pop();
//                 if size == 0 {
//                     println!();
//                     break;
//                 }
//                 entry.push_str(&input_tmp);
//                 let (input_tmp, open_quote2) = entry.custom_split();
//                 open_quote = open_quote2;
//                 command = input_tmp;
//                 if !open_quote {
//                     break;
//                 }
//             }
//         }

//         println!("command =>{command:?}");

//         if open_quote {
//             print_error("Syntax error: Unterminated quoted string");
//             continue;
//         }

//         if entry.split_whitespace().collect::<Vec<_>>().len() != 0 {
//             hist.push(entry.clone());
//         }

//         let mut first_pass_res: Vec<String> = Vec::new();

//         /* first pass */
//         // let temp = command.args

//         if !command.name.is_empty() {
//             first_pass_res.push(command.name);
//         }
//         for arg in command.args {
//             match arg {
//                 CommandPart::String(arg) => {
//                     if !arg.is_empty() {
//                         first_pass_res.push(arg)
//                     }
//                 }
//                 CommandPart::Substitution(sub_command) => {
//                     let mut sub_args: Vec<String> = Vec::new();
//                     for sub_arg in sub_command.args {
//                         if let CommandPart::String(sub_arg_str) = sub_arg {
//                             sub_args.push(sub_arg_str);
//                         }
//                     }

//                     if sub_command.name.is_empty() {
//                         continue;
//                     }
//                     let output = match sub_command.name.as_str() {
//                         "echo" => echo(&sub_args),
//                         "pwd" => pwd(&current_dir),
//                         "cd" => {
//                             cd(&sub_args, &mut current_dir, &mut history_current_dir, &home);
//                             "".to_string()
//                         }
//                         // "ls" => {
//                         //     ls(&sub_args, &current_dir);
//                         // }
//                         "cat" => cat(&sub_args, &current_dir),
//                         // "cp" => {
//                         //     cp(&sub_args);
//                         // }
//                         "rm" => {
//                             rm(&sub_args, &current_dir);
//                             "".to_string()
//                         }
//                         // "mv" => {
//                         //     mv(&sub_args);
//                         // }
//                         "mkdir" => {
//                             mkdir(&sub_args, &current_dir);
//                             "".to_string()
//                         }
//                         "history" => history(&hist),
//                         "exit" => exit(0),
//                         _ => {
//                             println!("\x1b[31mCommand '<{}>' not found\x1b[0m", sub_command.name);
//                             "".to_string()
//                         }
//                     };

//                     first_pass_res.push(output)
//                 }
//             }
//         }

//         /* second pass */
//         // println!("{:?}", first_pass_res);
//         let input = first_pass_res;
//         if input.is_empty() || input[0].is_empty() {
//             continue;
//         }

//         let command = input[0].as_str();
//         let args: Vec<String> = if input.len() > 1 {
//             input[1..].to_vec()
//         } else {
//             Vec::new()
//         };

//         match command {
//             "echo" => println!("{}", echo(&args)),
//             "pwd" => println!("{}", pwd(&current_dir)),
//             "cd" => {
//                 cd(&args, &mut current_dir, &mut history_current_dir, &home);
//             }
//             // "ls" => {
//             //     ls(&args, &current_dir);
//             // }
//             "cat" => println!("{}", cat(&args, &current_dir)),
//             // "cp" => {
//             //     cp(&args);
//             // }
//             "rm" => rm(&args, &current_dir),

//             // "mv" => {
//             //     mv(&args);
//             // }
//             "mkdir" => mkdir(&args, &current_dir),

//             "history" => println!("{}", history(&hist)),
//             "exit" => exit(0),
//             _ => println!("\x1b[31mCommand '<{command}>' not found\x1b[0m"),
//         };
//     }
// }

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
        "ls" => {
            ls(&args, &current_dir);
            None
        }
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
            println!("\x1b[31mCommand <{}> not found\x1b[0m", command);
            None
        }
    }
}

fn main() {
    println!("s\n");
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
