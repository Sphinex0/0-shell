use std::io::Write;
use std::io::stdin;

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
        let commands = [
            "echo", "cd", "ls", "pwd", "cat", "cp", "rm", "mv", "mkdir", "exit",
        ];
        match commands.contains(&command) {
            true => match command {
                "echo" => {
                    println!("edddd");
                }
                _ => {}
            },
            false => println!("err")
        }
    }
}
