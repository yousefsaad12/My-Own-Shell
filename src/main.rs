#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        let (command, text) = match line.split_once(' ') {
            Some((command, text)) => (command, text),
            None => (line, ""),
        };

        if command == "type" {

            if text == "echo" || text == "exit" 
            {
                println!("{} is a shell builtin", text)
            }
        }

        if command == "echo" {
            println!("{}", text)
        } else if command == "exit" {
            return;
        } else {
            println!("{}: command not found", command)
        }
    }
}
