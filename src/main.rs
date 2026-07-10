#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        let (command, text) = match line.split_once(' ') {
            Some((command, text)) => (command, text),
            None => (line, ""),
        };

        if command == "echo" {
            println!("{}", text)
        };
 
    return ;
}
