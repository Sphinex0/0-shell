// use std::io;

// fn main() {
//    loop {
//     print!("$");
//     let mut input: String = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
    
//     let input = input.trim();
//     if input.is_empty() {
//         continue;
//     }

//         let args: Vec<&str> = input.split_whitespace().collect();

//         let command = args.get(0).unwrap_or(&"");
//         let arguments = &args[1..];

//         match *command {
//             "exit" => {
//                 println!("Exiting shell...");
//                 break;
//             }
//             "ls" => {
//                 let mut cmd = Command::new("ls");
//                 if !arguments.is_empty() {
//                     cmd.args(arguments);
//                 }
//                 match cmd.output() {
//                     Ok(output) => {
//                         io::stdout().write_all(&output.stdout).expect("Failed to write output");
//                         io::stderr().write_all(&output.stderr).expect("Failed to write error");
//                     }
//                     Err(e) => eprintln!("Error executing ls: {}", e),
//                 }
//             }
//             "echo" => {
//                 println!("{}", arguments.join(" "));
//             }
//             "" => {} /
//             _ => {
//                 let mut cmd = Command::new(command);
//                 if !arguments.is_empty() {
//                     cmd.args(arguments);
//                 }
//                 match cmd.output() {
//                     Ok(output) => {
//                         io::stdout().write_all(&output.stdout).expect("Failed to write output");
//                         io::stderr().write_all(&output.stderr).expect("Failed to write error");
//                     }
//                     Err(_) => {
//                         println!("minishell: command not found: {}", command);
//                     }
//                 }
//             }
//         }
//     }
// }