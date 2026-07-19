use std::io::{self, Write};
use crate::commands::{echo, exit, type_cmd};
use crate::external::{finder, executor};

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "echo" => echo::run(&args),
            "exit" => exit::run(),
            "type" => type_cmd::run(args.first().copied().unwrap_or("")),
            "" => {}
            _ =>{
                match finder::find_exe(command){
                    Some(path) => executor::run(&path,command, args),
                    None => println!("{}: command not found", command),
                }
            }
        }
    }
}